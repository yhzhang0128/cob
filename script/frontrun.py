import time
import socket

server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_socket.bind(('control', 30000))
NCLIENT=2
server_socket.listen(NCLIENT)
print("Server is listening on control:30000...")

try:
    client_socket1, client_address1 = server_socket.accept()
    print(f"Accepted connection from {client_address1}")

    client_socket2, client_address2 = server_socket.accept()
    print(f"Accepted connection from {client_address2}")

    binary_reply = b'\x00\x00\x00\x00\x0f\x01\x00\x00\x00\xad\xc8;\x19\n'
    try:
        while True:
            data1 = client_socket1.recv(1024)
            data2 = client_socket2.recv(1024)
            if data1:
                print(f"Received: {data1}")
            if data2:
                print(f"Received: {data2}")

            time.sleep(0.4)
            client_socket1.sendall(binary_reply)
            time.sleep(0.01) # Let client1 frontrun 10ms.
            client_socket2.sendall(binary_reply)
            print("Control tells both to proceed.");
    finally:
        client_socket1.close()
        client_socket2.close()
finally:
    server_socket.close()
