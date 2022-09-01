use std::env;

fn main() {
    let os = env::consts::OS.to_string();
    let arch = env::consts::ARCH.to_string();

    println!("os {}", os);
    println!("arch {}", arch);
}
