use tokio::sync::mpsc;
use tokio::time::{Duration, sleep, Instant};

struct Message {
    worker_name: String,
    value: u32,
}

impl Message {
    fn new(worker_name: String, value: u32) -> Message {
        Message { worker_name, value }
    }
}

async fn worker(name: String, tx: mpsc::Sender<Message>) {
    for i in 0..4 {
        tx.send(Message::new(name.clone(), i)).await.unwrap();
    }    
}

#[tokio::main]
async fn main() {

    // A
    {
        let (tx, mut rx) = mpsc::channel::<u32>(10);

        tokio::spawn(async move {
            for i in 0..=4 {
                tx.send(i).await.unwrap();
                sleep(Duration::from_millis(500)).await;
            }
        });

        let mut values: Vec<u32> = Vec::new();

        while let Some(i) = rx.recv().await {
            values.push(i);
        }

        let sum = values.iter().sum::<u32>();
        let count = values.len();
        let average = sum as f32 / count as f32;

        println!("Somme des valeurs envoyées: {}", sum);
        println!("Moyenne des valeurs envoyées: {:?}", average);
    }

    println!();

    // B
    {
        let (tx, mut rx) = mpsc::channel::<Message>(10);

        let tx1 = tx.clone();
        let tx2 = tx.clone();
        let tx3 = tx.clone();

        let _spawn1 = tokio::spawn(async move {
            worker("worker1".to_string(), tx1).await;
        });

        let _spawn2 = tokio::spawn(async move {
            worker("worker2".to_string(), tx2).await;
        });

        let _spawn3 = tokio::spawn(async move {
            worker("worker3".to_string(), tx3).await;
        });

        drop(tx);

        let mut messages: Vec<Message> = Vec::new();

        while let Some(msg) = rx.recv().await {
            println!("{} envoyé {}", msg.worker_name, msg.value);
            messages.push(msg);
        }

        let message_worker_1 = messages.iter().filter(|v| v.worker_name == "worker1").count();
        let message_worker_2 = messages.iter().filter(|v| v.worker_name == "worker2").count();
        let message_worker_3 = messages.iter().filter(|v| v.worker_name == "worker3").count();

        println!("{}", message_worker_1);
        println!("{}", message_worker_2);
        println!("{}", message_worker_3);
    }

    println!();

    // C
    {
        let now = Instant::now();

        let (tx, mut rx) = mpsc::channel::<u32>(2);

        tokio::spawn(async move {
            for i in 0..=5 {
                println!("Before send: {:?}", now.elapsed());
                tx.send(i).await.unwrap();
                println!("After send: {:?}", now.elapsed());
            }
        });

        while let Some(i) = rx.recv().await {
            println!("{}", i);
            sleep(Duration::from_millis(500)).await
        }
    }
}
