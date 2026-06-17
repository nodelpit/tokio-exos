use tokio::time::{sleep, Duration, Instant};

async fn task(duration: Duration) {
    sleep(duration).await;
}

fn calcul(x: u8, y: u8) -> String {
    if x + y > 100 {
        "Opération impossible".to_string()
    } else {
        "OK".to_string()
    }
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let spawn_task_1 = tokio::spawn(async move {
        task(Duration::from_millis(500)).await;
    });

    let spawn_task_2 = tokio::spawn(async move {
        task(Duration::from_millis(800)).await;
    });

    let spawn_task_3 = tokio::spawn(async move {
        task(Duration::from_millis(200)).await;
    });

    let spawn_calcul = tokio::spawn(async move {
        calcul(3, 99)
    });
    
    let panic_task = tokio::spawn(async move {
        panic!("Intentional panic !");
    });
    
    match spawn_calcul.await {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }

    match panic_task.await {
        Ok(_) => println!("task OK"),
        Err(e) => println!("{:?} task panic !", e),
    }
    
    spawn_task_1.await.expect("Task 1 panicked");
    spawn_task_2.await.expect("Task 2 panicked");
    spawn_task_3.await.expect("Task 3 panicked");

    println!("---- durée total : {:?}", start.elapsed());
}