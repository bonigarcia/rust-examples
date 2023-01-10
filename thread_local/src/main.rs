use std::cell::RefCell;

thread_local!(static GLOBAL_DATA: RefCell<String> = RefCell::new("Original message".to_string()));

fn main() {
    GLOBAL_DATA.with(|text| {
        println!("Global string is {}", *text.borrow());
    });

    GLOBAL_DATA.with(|text| {
        *text.borrow_mut() = "New message".to_string();
    });

    GLOBAL_DATA.with(|text| {
        println!("Global string is {}", *text.borrow());
    });
}
