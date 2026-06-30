use std::pin::Pin;
use std::task::{Context, Poll};

struct Ready<T> {
    value: Option<T>,
}

struct CountdownFuture {
    count: u32,
}

impl<T: Unpin> Future for Ready<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        let value = match this.value.take() {
            Some(v) => v,
            None => panic!("future already completed"),
        };

        Poll::Ready(value)
    }
}

impl<T> Ready<T> {
    fn new(value: T) -> Self {
        Self { value: Some(value) }
    }
}

impl CountdownFuture {
    fn new(count: u32) -> Self {
        Self { count }
    }
}

impl Future for CountdownFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();

        println!("poll, count = {}", this.count);

        if this.count > 1 {
            this.count -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

#[tokio::main]

async fn main() {
    let value = Ready::new(20);

    println!("value: {}", value.await);

    let value1 = CountdownFuture::new(5);

    println!("value: {:?}", value1.await);
}
