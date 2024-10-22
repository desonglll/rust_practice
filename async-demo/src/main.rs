use std::{
    future::Future,
    sync::{Arc, Mutex},
    task::Poll,
    thread,
    time::Duration,
};

struct Point {
    x: Arc<Mutex<i32>>,
}

impl Future for Point {
    type Output = i32;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let x = self.x.lock().unwrap();
        let x_thread = self.x.clone();
        if *x == 10 {
            println!("Ready");
            Poll::Ready(*x)
        } else {
            let waker = cx.waker().clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(2));
                let mut x = x_thread.lock().unwrap();
                *x += 1;
                waker.wake();
            });
            println!("Pending {:?}", x);
            Poll::Pending
        }
    }
}
#[allow(clippy::needless_return)]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let future = Point {
        x: Arc::new(Mutex::new(2)),
    };
    let result = future.await;
    println!("Result: {}", result);
    Ok(())
}
