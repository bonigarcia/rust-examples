use std::path::Path;

use is_executable::IsExecutable;

fn main() {
    //let path = Path::new("C:\\Users\\boni\\AppData\\Roaming\\npm\\chromedriver");
    //let path = Path::new("C:\\Users\\boni\\.cache\\selenium\\chromedriver\\win32\\111.0.5563.64\\chromedriver.exe");
    let path = Path::new("C:\\Users\\boni\\AppData\\Roaming\\npm\\chromedriver.cmd");

    if path.is_executable() {
        println!("The path is executable!");
    } else {
        println!("The path is _not_ executable!");
    }
}
