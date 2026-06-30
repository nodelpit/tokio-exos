# 🏁 tokio-exos

Exercices progressifs pour apprendre le modèle asynchrone de Rust avec Tokio.

---

## Exercices

### 01 · runtime_and_async
Comportement séquentiel vs concurrent. Le runtime Tokio n'exécute pas les `await` en parallèle par défaut : chaque `.await` attend la complétion avant de passer à la suite.

**Concepts :** `#[tokio::main]`, `async/await`, `sleep`, `Instant::now()`

---

### 02 · spawn_and_join
`tokio::spawn` lance une tâche sur le runtime sans bloquer le thread courant. `JoinHandle::await` synchronise à la fin. Un panic dans une tâche n'est pas fatal : le `JoinHandle` renvoie `Err(JoinError)`.

**Concepts :** `tokio::spawn`, `JoinHandle`, gestion des panics

---

### 03 · try_join_and_try
`join!` attend toutes les futures en parallèle et renvoie leurs résultats en tuple. `try_join!` court-circuite dès la première `Err`.

**Concepts :** `tokio::join!`, `tokio::try_join!`, parallélisme sans spawn

---

### 04 · mpsc_canal
Canal multi-producteur / mono-consommateur. Le `Sender` est `Clone`, le `Receiver` est unique. Fermer tous les `Sender` ferme le canal et fait sortir la boucle `while let Some`.

**Concepts :** `tokio::sync::mpsc`, backpressure (capacité bornée), fan-in

---

### 05 · oneshot_and_timeout
`oneshot` envoie exactement une valeur d'une tâche vers une autre (réponse de job). `timeout` enveloppe n'importe quelle future et renvoie `Err(Elapsed)` si elle dépasse le délai.

**Concepts :** `tokio::sync::oneshot`, `tokio::time::timeout`, pattern requête/réponse

---

### 06 · select
`select!` écoute plusieurs futures simultanément et prend la première qui est prête. Les guards (`if !paused`) désactivent une branche conditionnellement. La branche priority est listée en premier → avantage en cas de compétition.

**Concepts :** `tokio::select!`, guards, canaux de priorité et de pause

---

### 07 · arc_mutex
`Arc<Mutex<T>>` permet à plusieurs tâches de partager et muter un même état. `Arc::clone` partage la référence comptée, `.lock().unwrap()` acquiert le verrou.

**Concepts :** `Arc`, `std::sync::Mutex` (vs `tokio::sync::Mutex`), agrégation de métriques

---

### 08 · spawn_blocking
Les calculs CPU lourds bloquent le thread et affament le runtime. `spawn_blocking` les déporte sur un thread dédié hors du pool async.

**Concepts :** `tokio::task::spawn_blocking`, thread pool séparé, coût d'un blocage sur le runtime

---

### 09 · implement_future
Implémenter `Future` à la main : `poll` renvoie `Poll::Pending` ou `Poll::Ready`. `cx.waker().wake_by_ref()` demande au runtime de repolluter immédiatement.

**Concepts :** trait `Future`, `Poll`, `Pin`, `Waker`, `Context`

---

### 10 · tcp_server
Serveur TCP echo concurrent avec limite de connexions (`Semaphore`), compteur RAII (`Drop` sur `ConnGuard`) et arrêt gracieux via canal `broadcast`.

**Concepts :** `TcpListener`, `AsyncReadExt/AsyncWriteExt`, `Semaphore`, `broadcast`, shutdown pattern
