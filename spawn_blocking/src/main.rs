use std::collections::{BTreeMap, HashMap};
use std::thread;

use tokio::task::spawn_blocking;
use tokio::time::{Duration, Instant, sleep};

fn load_config() -> HashMap<String, String> {
    thread::sleep(Duration::from_millis(200));

    let mut hash = HashMap::new();

    hash.insert("key-1".to_string(), "value-1".to_string());
    hash.insert("key-2".to_string(), "value-2".to_string());
    hash.insert("key-3".to_string(), "value-3".to_string());

    hash
}

#[tokio::main]
async fn main() {
    println!("DÉBUT DES WORKERS");
    let now = Instant::now();

    let heavy = spawn_blocking(|| {
        let now = Instant::now();

        let result: u64 = (0..500_000_000).sum();
        println!(
            "heavy result: {} - Temps écoulé: {:?}",
            result,
            now.elapsed()
        );
    });

    let config_handle = spawn_blocking(load_config);

    let light1 = async {
        let now = Instant::now();
        sleep(Duration::from_millis(10)).await;
        println!("Temps écoulé: {:?}", now.elapsed(),);
    };

    let light2 = async {
        let now = Instant::now();
        sleep(Duration::from_millis(10)).await;
        println!("Temps écoulé: {:?}", now.elapsed(),);
    };

    let light3 = async {
        let now = Instant::now();
        sleep(Duration::from_millis(10)).await;
        println!("Temps écoulé: {:?}", now.elapsed(),);
    };

    tokio::join!(heavy, light1, light2, light3);
    let config = config_handle.await.unwrap();

    let sorted: BTreeMap<_, _> = config.into_iter().collect();

    for (k, v) in sorted {
        println!("{k} -> {v}");
    }

    println!("FIN DES WORKERS - TEMPS TOTAL; {:?}", now.elapsed());
}
