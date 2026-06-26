use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    {
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<String>(5);

        let (priority_tx, mut priority_rx) = mpsc::channel::<String>(5);

        let (pause_tx, mut pause_rx) = mpsc::channel::<bool>(5);

        tokio::spawn(async move {
            for i in 0..=5 {
                cmd_tx.send(i.to_string()).await.unwrap();
                sleep(Duration::from_millis(200)).await;
            }
        });

        tokio::spawn(async move {
            for i in 0..=3 {
                priority_tx.send(i.to_string()).await.unwrap();
                sleep(Duration::from_millis(360)).await;
            }
        });

        tokio::spawn(async move {
            sleep(Duration::from_millis(300)).await;
            pause_tx.send(true).await.unwrap();
            sleep(Duration::from_millis(700)).await;
            pause_tx.send(false).await.unwrap();
        });

        let mut cmd_closed = false;
        let mut priority_closed = false;
        let mut paused = false;

        loop {
            tokio::select! {
                v = cmd_rx.recv(), if !paused => {
                    match v {
                        Some(cmd) => {
                            println!("[normal] {}", cmd);
                        }
                        None => {
                            cmd_closed = true;
                        }
                    }
                }
                v = priority_rx.recv() => {
                    match v {
                        Some(cmd) => {
                            println!("[priority] {}", cmd);
                        }
                        None => {
                            priority_closed = true;
                        }
                    }
                }
                v = pause_rx.recv() => {
                    match v {
                        Some(cmd) => {
                            paused = cmd;
                            println!("== PAUSE STATE: {} ==", paused);
                        }
                        None => {}
                    }
                }
            }

            if cmd_closed && priority_closed {
                break;
            }
        }
    }
}
