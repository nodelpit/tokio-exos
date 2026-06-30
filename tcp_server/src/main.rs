use std::io;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::{Semaphore, broadcast};
use tokio::time::{Duration, sleep};

struct ConnGuard {
    counter: Arc<Mutex<u32>>,
}

impl Drop for ConnGuard {
    fn drop(&mut self) {
        let mut c = self.counter.lock().unwrap();
        *c -= 1;
        println!("Connections actives : {}", *c);
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let semaphore = Arc::new(Semaphore::new(3));
    let counter = Arc::new(Mutex::new(0u32));

    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    loop {
        tokio::select! {
            res = listener.accept() => {
                let (mut socket, _) = match res {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("accept error: {e}");
                        continue;
                    }
                };

                let semaphore = semaphore.clone();
                let counter = counter.clone();
                let mut shutdown_rx = shutdown_tx.subscribe();

                tokio::spawn(async move {
                    let _permit = semaphore.acquire_owned().await.unwrap();

                    {
                        let mut c = counter.lock().unwrap();
                        *c += 1;
                        println!("Connections actives : {}", *c);
                    }

                    let _guard = ConnGuard {
                        counter: counter,
                    };

                    let mut buf = vec![0u8; 1024];

                    loop {
                        tokio::select! {
                            res = socket.read(&mut buf) => {
                                match res {
                                    Ok(0) => break,
                                    Ok(n) => {
                                        if socket.write_all(&buf[..n]).await.is_err() {
                                            break;
                                        }
                                    }
                                    Err(_) => break,
                                }
                            }

                            _ = shutdown_rx.recv() => {
                                break;
                            }
                        }
                    }
                });
            }

            _ = tokio::signal::ctrl_c() => {
                break;
            }
        }
    }

    let _ = shutdown_tx.send(());

    loop {
        let active = *counter.lock().unwrap();
        if active == 0 {
            break;
        }
        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
