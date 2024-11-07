import http.client

def send_chunked_mp4(file_path, server, port, method, endpoint):
    # Initialize the HTTP connection with chunked transfer encoding.
    conn = http.client.HTTPConnection(server, port)

    # Set headers for the request, specifying Transfer-Encoding: chunked.
    headers = {
        "Content-Type": "video/mp4",
        "Transfer-Encoding": "chunked"
    }

    # Open the file in binary mode for reading.
    with open(file_path, "rb") as file:
        # Initiate a request with chunked encoding.
        conn.putrequest(method, endpoint)
        for header, value in headers.items():
            conn.putheader(header, value)
        conn.endheaders()

        # Define chunk size (e.g., 8192 bytes).
        chunk_size = 8192
        sent = 0
        while True:
            # Read a chunk from the file.
            chunk = file.read(chunk_size)
            if not chunk:
                # End of file reached, send zero-length chunk to signal end of transfer.
                conn.send(b"0\r\n\r\n")
                break
            # Send the length of the chunk in hexadecimal, followed by the chunk data.
            conn.send(f"{len(chunk):X}\r\n".encode("utf-8"))
            conn.send(chunk)
            conn.send(b"\r\n")  # Each chunk ends with a CRLF.
            print("Chunk sent")
            sent += chunk_size
            print(f"Sent {sent} bytes")


    # Get the response from the server.
    response = conn.getresponse()
    print("Status:", response.status)
    print("Reason:", response.reason)
    print("Response:", response.read().decode("utf-8"))

    # Close the connection.
    conn.close()

# Example usage
file_path = "C://Users/vojtech.kopal/Videos/test.mp4"
server = "localhost"
port = 3000
endpoint = "/test.mp4"
send_chunked_mp4(file_path, server, port, method="PUT", endpoint=endpoint)
