use std::error::Error;
use std::time::Duration;
use tokio;

async fn task1() -> Result<String, Box<dyn Error>> {
    for i in 0..10 {
        println!("Task 1: {}", i);
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    Ok("Task 1 OK".into())
    //Err("Error in task 1".into())
}

async fn task2() -> Result<String, Box<dyn Error>> {
    for i in 0..10 {
        println!("Task 2: {}", i);
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    Ok("Task 2 OK".into())
    //Err("Error in task 2".into())
}

#[tokio::main]
async fn main() {
    let t1 = task1();
    let t2 = task2();

    match tokio::try_join!(t1, t2) {
        Ok(res) => {
            println!("All good: {} -- {}", res.0, res.1);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
