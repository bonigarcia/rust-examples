use std::rc::Rc;

fn main() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let u: Rc<String> = s.clone();

    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
}
