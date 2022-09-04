fn main() {
    let a = 1;
    let b = 2;
    let c = my_sum(a, b);

    println!("The sum of {a} and {b} is {c}");
}

fn my_sum(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(5, my_sum(2, 3));
    }
}