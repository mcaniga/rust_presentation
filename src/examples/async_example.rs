use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

async fn run_tokio_example() -> io::Result<()> {
    // await - non-blocking, pauses execution of this function, freeing up the thread
    // waiting - blocking, current thread is waiting instead of doing smth usefull
    let mut buffer = File::create("foo.txt").await?;
    buffer.write_all(b"some bytes").await?;
    println!("File was created");
    Ok(())
}

pub async fn run_async_demo() {
    let res = run_tokio_example().await;
    res.unwrap();
}
