use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // Return value of `Ok(0)` means that the remote
                    // has closed.
                    Ok(0) => return,
                    Ok(n) => {
                        // Copy the data back to the socket
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // Unexpected error, nothing to really do here
                            // so just stop processsing.
                            return;
                        }
                    }
                    Err(_) => {
                        // Unexpected socket error. Nothing to really do here
                        // so just stop processsing.
                        return;
                    }
                }
            }
        });
    }
}