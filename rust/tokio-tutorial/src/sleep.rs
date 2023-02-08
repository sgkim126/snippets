use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("2");
        if Instant::now() >= self.when {
            println!("5");
            Poll::Ready(())
        } else {
            let waker = cx.waker().clone();
            let when = self.when;

            println!("3");
            thread::spawn(move || {
                println!("5");
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }
                println!("6");

                waker.wake();
                println!("7");
            });
            println!("4");

            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(1);
    let future = Delay { when };
    println!("1");

    future.await;
    println!("8");
}
