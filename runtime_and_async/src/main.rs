use tokio::time::{sleep, Duration, Instant};

// async fn task_a(name: String) {
//     println!("{} just Starting", name);
//     let _ = sleep(Duration::from_secs(1)).await;
//     println!("{} just finished", name);
// }

// async fn task_b(name: String) {
//     println!("{} just Starting", name);
//     let _ = sleep(Duration::from_secs(2)).await;
//     println!("{} just finished", name);
// }

// async fn task_c(name: String) {
//     println!("{} just Starting", name);
//     let _ = sleep(Duration::from_millis(500)).await;
//     println!("{} just finished", name);
// }

async fn task (name: String, duration: Duration) {
    println!("{} just Starting", name);
    sleep(duration).await;
    println!("{} just finished", name);
}

#[tokio::main]
async fn main() {
    // let now_a = Instant::now(); // Capture the time of when the fn started
    // task_a("task_a".to_string()).await;
    // println!("durée : {:?}", now_a.elapsed()); // Show how long the fn took to complete

    // let now_b = Instant::now();
    // task_b("task_b".to_string()).await;
    // println!("durée : {:?}", now_b.elapsed());

    // let now_c = Instant::now();
    // task_c("task_c".to_string()).await;
    // println!("durée : {:?}", now_c.elapsed());
    
    let now_3 = Instant::now();
    task("Principal task".to_string(), Duration::from_millis(500)).await;
    println!("---- durée : {:?}", now_3.elapsed());

    let now_1 = Instant::now();
    task("Second task".to_string(), Duration::from_secs(1)).await;
    println!("---- durée : {:?}", now_1.elapsed());

    let now_2 = Instant::now();
    task("Third task".to_string(), Duration::from_secs(2)).await;
    println!("---- durée : {:?}", now_2.elapsed());

}