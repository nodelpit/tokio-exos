use tokio::time::{sleep, Duration, Instant};

async fn fetch_score(player: &str, delay: Duration, score: u32) -> (String, u32) {
    sleep(delay).await;
    (player.to_string(), score)
}

async fn validate_score(score: u32) -> Result<u32, String> {
    if score > 100 {
        Err("score invalide".to_string())
    } else {
        Ok(score)
    }
}

#[tokio::main]
async fn main() {
    // A
    let now = Instant::now();

    let (first, second, third) = tokio::join!(
        fetch_score("player-1", Duration::from_millis(500), 47),
        fetch_score("player-2", Duration::from_millis(700), 65),
        fetch_score("player-3", Duration::from_millis(900), 52),
    );

    let v = vec![first.1, second.1, third.1];
    
    let total: u32 = v.iter().sum();
    let max = v.iter().max();
    let treshold: Vec<u32> = v.iter().filter(|x| **x > 50).copied().collect();

    println!("somme des scores: {}", total);
    println!("score le plus haut: {:?}", max);
    println!("score dépassant le seuil: {:?}", treshold);

    println!("---- durée de la partie A : {:?}", now.elapsed());

    println!();

    //  B
    let now = Instant::now();

    // Retourne une erreur dès qu une Future échoue
    let res = tokio::try_join!(
        validate_score(67),
        validate_score(121),
    );

    match res {
        Ok((first, second)) => {
            println!("Scores valides : {}, {}", first, second);
        }
        Err(e) => {
            println!("Erreur : {}", e)
        }
    }

    println!("---- durée de la partie B (join!) : {:?}", now.elapsed());

    // C
    let now = Instant::now();

    let t1 = tokio::spawn(async move {
        fetch_score("player-4", Duration::from_millis(500), 47).await;
    });
    let t2 = tokio::spawn(async move {
        fetch_score("player-5", Duration::from_millis(700), 65).await;
    });
    let t3 = tokio::spawn(async move {
        fetch_score("player-6", Duration::from_millis(900), 52).await;
    });

    t1.await.unwrap();
    t2.await.unwrap();
    t3.await.unwrap();

    println!("---- durée de la partie C (spawn) : {:?}", now.elapsed());
}