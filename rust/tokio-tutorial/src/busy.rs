use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
    count: AtomicUsize,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("2");
        if Instant::now() >= self.when {
            println!("5");
            Poll::Ready(())
        } else {
            static COUNT: AtomicUsize = AtomicUsize::new(0);
            println!(
                "3 {} {}",
                self.count.fetch_add(1, Ordering::SeqCst),
                COUNT.fetch_add(1, Ordering::SeqCst)
            );
            cx.waker().wake_by_ref();
            println!("4");
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(1);
    let future = Delay {
        when,
        count: AtomicUsize::new(0),
    };
    println!("1");

    future.await;
    println!("6");
}
