use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wr) = io::split(socket);

    // Write data in the background
    tokio::spawn(async move {
        wr.write_all(b"Hello").await?;
        wr.write_all(b"World!").await?;

        Ok::<_, io::Error>(())
    });

    let mut buffer = vec![0; 128];

    loop {
        let n = rd.read(&mut buffer).await?;

        if n == 0 {
            break;
        }

        println!("GOT {:?}", &buffer[..n]);
    }

    Ok(())
}