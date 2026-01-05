use crate::drivers::net::{NET_IFACE, NET_DEVICE, SOCKETS};
use smoltcp::time::Instant;
use smoltcp::socket::tcp::{Socket as TcpSocket, SocketBuffer as TcpSocketBuffer};
use smoltcp::iface::SocketHandle;
use alloc::vec;
use crate::serial_println;
use core::sync::atomic::{AtomicU64, Ordering};

static TIME: AtomicU64 = AtomicU64::new(0);

// Simple monotonic timer since we don't have high precision hardware timer hooked up to smoltcp yet
fn current_time() -> Instant {
    Instant::from_millis(TIME.fetch_add(10, Ordering::Relaxed) as i64)
}

pub async fn poll_task() {
    let mut counter: u64 = 0; // Initialize counter outside the main polling loop
    loop {
        counter += 1;
        if counter % 1000 == 0 {
             crate::serial_println!("Net Polling... {}", counter);
        }
        
        {
            let mut iface_lock = crate::drivers::net::NET_IFACE.lock();
            let mut device_lock = crate::drivers::net::NET_DEVICE.lock();
            let mut sockets_lock = SOCKETS.lock();
            
            if let Some(iface) = iface_lock.as_mut() {
                if let Some(device) = device_lock.as_mut() {
                     let time = current_time();
                     match iface.poll(time, device, &mut sockets_lock) {
                        true => {
                            // Activity occurred
                        },
                        false => {
                            // No activity
                        }
                     }
                }
            }
        }
        YieldNow::default().await;
    }
}


// Improved Logic with Handle
// use smoltcp::iface::SocketHandle; // Already imported at top? Or assume it uses the top one.

pub async fn server_task() {
    let handle: SocketHandle;
    {
        let mut sockets = SOCKETS.lock();
        let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
        let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
        let socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);
        handle = sockets.add(socket);
    }
    
    let _is_open = false;

    loop {
        {
            let mut sockets = SOCKETS.lock();
            let socket = sockets.get_mut::<TcpSocket>(handle);
            
            if !socket.is_open() {
                // Listen
                socket.listen(8023).ok(); // Port 8023
                crate::serial_println!("[Net] Listening on :8023");
            }
            
            if socket.can_send() && socket.can_recv() {
                // Simple Echo/Shell
                // Read input
                let mut data = [0u8; 128];
                match socket.recv_slice(&mut data) {
                    Ok(size) if size > 0 => {
                         let s = core::str::from_utf8(&data[..size]).unwrap_or("?");
                         crate::serial_println!("[Remote] {}", s);
                         // Echo + response (Mock Shell)
                         use alloc::format;
                         let response = format!("SawitRemote> You said: {}\n", s.trim());
                         socket.send_slice(response.as_bytes()).ok();
                         
                         // Here we could bridge to shell commands
                         // ...
                    }
                    _ => {}
                }
            }
        }
        YieldNow::default().await;
    }
}


// Utility Future to Yield
#[derive(Default)]
struct YieldNow {
    yielded: bool,
}

impl core::future::Future for YieldNow {
    type Output = ();
    fn poll(mut self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<()> {
        if self.yielded {
            core::task::Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            core::task::Poll::Pending
        }
    }
}
