use std::env;

fn main() {
    let os = my_os();
    let arch = my_arch();

    println!("os {}", os);
    println!("arch {}", arch);
}

fn my_arch() -> String {
    env::consts::ARCH.to_string()
}

fn my_os() -> String {
    env::consts::OS.to_string()
}

#[test]
fn test_os() {
    assert!( ["linux", "windows", "macos"].contains(&&*my_os()));
}

#[test]
fn test_arch() {
    assert_eq!("x86_64", my_arch());
}
