use tokio::sync::{mpsc, oneshot};
use tokio::time::{Duration, sleep, timeout};

struct Job {
    id: u32,
    duration: Duration,
    reply_to: oneshot::Sender<String>,
}

impl Job {
    fn new(id: u32, duration: Duration, reply_to: oneshot::Sender<String>) -> Job {
        Job {
            id,
            duration,
            reply_to,
        }
    }

    async fn worker(mut rx: mpsc::Receiver<Job>) {
        while let Some(job) = rx.recv().await {
            sleep(job.duration).await;
            let msg = format!("Job {} terminé en {:?}", job.id, job.duration);
            match job.reply_to.send(msg) {
                Ok(_) => println!("job {} envoyé", job.id),
                Err(e) => println!("job {} : receiver disparu -> {}", job.id, e),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    {
        let (tx, rx) = mpsc::channel::<Job>(5);

        let handle = tokio::spawn(async move {
            Job::worker(rx).await;
        });

        let (reply1, rx1) = oneshot::channel::<String>();
        let job1 = Job::new(1, Duration::from_secs_f64(1.2), reply1);
        tx.send(job1).await.unwrap();

        let (reply2, rx2) = oneshot::channel::<String>();
        let job2 = Job::new(2, Duration::from_secs_f64(1.2), reply2);
        tx.send(job2).await.unwrap();

        let (reply3, rx3) = oneshot::channel::<String>();
        let job3 = Job::new(3, Duration::from_secs_f64(1.2), reply3);
        tx.send(job3).await.unwrap();

        let (reply4, rx4) = oneshot::channel::<String>();
        let job4 = Job::new(4, Duration::from_secs(3), reply4);
        tx.send(job4).await.unwrap();

        let (reply5, rx5) = oneshot::channel::<String>();
        let job5 = Job::new(5, Duration::from_secs(5), reply5);
        tx.send(job5).await.unwrap();

        sleep(Duration::from_millis(100)).await;
        drop(rx5);

        let (r1, r2, r3) = tokio::join!(rx1, rx2, rx3);
        println!("{}", r1.unwrap());
        println!("{}", r2.unwrap());
        println!("{}", r3.unwrap());

        let r4: Option<String> = match timeout(Duration::from_secs(1), rx4).await {
            Ok(Ok(msg)) => {
                println!("réponse reçue : {}", msg);
                Some(msg)
            }
            Ok(Err(_)) => {
                println!("sender droppé");
                None
            }
            Err(_) => {
                println!("timeout sur job 4");
                None
            }
        };

        drop(tx);
        handle.await.unwrap();

        println!("{:?}", r4);
    }
}
