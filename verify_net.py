import socket
import time
import sys

def verify_connection():
    target_host = "127.0.0.1"
    target_port = 8023
    
    print(f"Attempting to connect to {target_host}:{target_port}...")
    
    for i in range(30):
        try:
            client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            client.settimeout(5)
            client.connect((target_host, target_port))
            print("Connected!")
            
            message = "Hello SawitCore"
            print(f"Sending: {message}")
            client.send(message.encode('utf-8'))
            
            response = client.recv(4096)
            decoded = response.decode('utf-8')
            print(f"Received: {decoded}")
            
            if "SawitRemote> You said: Hello SawitCore" in decoded:
                print("VERIFICATION SUCCESS: TCP Echo working correctly.")
                sys.exit(0)
            else:
                print("VERIFICATION FAILED: Unexpected response.")
                sys.exit(1)
                
        except Exception as e:
            print(f"Connection failed (attempt {i+1}/10): {e}")
            time.sleep(1)
            
    print("VERIFICATION FAILED: Could not connect after 10 attempts.")
    sys.exit(1)

if __name__ == "__main__":
    verify_connection()
