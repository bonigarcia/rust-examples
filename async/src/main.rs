use std::time::Duration;
use tokio;

async fn task1() {
    for i in 0..10 {
        println!("Task 1: {}", i);
        tokio::time::sleep (Duration::from_millis (1)).await;
    }
}

async fn task2() {
    for i in 0..10 {
        println!("Task 2: {}", i);
        tokio::time::sleep (Duration::from_millis (1)).await;
    }
}

#[tokio::main]
async fn main() {
    let t1 = task1();
    let t2 = task2();
    tokio::join!(t1, t2);
}