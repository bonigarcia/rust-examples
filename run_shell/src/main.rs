use std::error::Error;
use std::process::Command;

pub const CRLF: &str = "\r\n";
pub const LF: &str = "\n";

fn main() -> Result<(), Box<dyn Error>> {
    let powershell_commands = vec![
        r#"(Get-Item 'C:\Program Files\Google\Chrome\Application\chrome.exe').VersionInfo.ProductVersion"#.to_string(),
        r#"(Get-Item "C:\Program Files\Google\Chrome\Application\chrome.exe").VersionInfo.ProductVersion"#.to_string(),
        r#"(Get-Item "$Env:Programfiles\Google\Chrome\Application\chrome.exe").VersionInfo.ProductVersion"#.to_string(),
        r#"echo "$Env:Programfiles""#.to_string(),
    ];
    for command in powershell_commands {
        print!("{} -- ", command);
        let output = run_powershell_command("windows", command)?;
        println!("{}", output);
    }

    let cmd_commands = vec![
        r#"echo %PROGRAMFILES%"#.to_string(),
        r#"echo %PROGRAMFILES: (x86)=%****"#.to_string(),
        r#"echo %PROGRAMFILES(X86)%"#.to_string(),
        r#"SET "PFILES=%PROGRAMFILES: (x86)=%" && call echo %^PFILES%*****"#.to_string(),
        r#"SET "PFILES=%PROGRAMFILES: (x86)=%" && call echo %^PFILES:\=\\%*****"#.to_string(),
        r#"SET "PFILES=%PROGRAMFILES: (x86)=%" && call wmic datafile where name='%^PFILES:\=\\%\\Mozilla Firefox\\firefox.exe' get Version /value"#.to_string(),
    ];
    for command in cmd_commands {
        print!("{} -- ", command);
        let output = run_shell_command("windows", command)?;
        println!("{}", output);
    }

    Ok(())
}

pub fn run_shell_command(os: &str, command: String) -> Result<String, Box<dyn Error>> {
    let (shell, flag) = if os.eq_ignore_ascii_case("windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };
    let output = Command::new(shell)
        .args([flag, command.as_str()])
        .output()?;
    Ok(
        strip_trailing_newline(String::from_utf8_lossy(&output.stdout).to_string().as_str())
            .to_string(),
    )
}

pub fn run_powershell_command(os: &str, command: String) -> Result<String, Box<dyn Error>> {
    let (shell, flag) = if os.eq_ignore_ascii_case("windows") {
        ("powershell", "-c")
    } else {
        ("sh", "-c")
    };
    let output = Command::new(shell)
        .args([flag, command.as_str()])
        .output()?;
    Ok(
        strip_trailing_newline(String::from_utf8_lossy(&output.stdout).to_string().as_str())
            .to_string(),
    )
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix(CRLF)
        .or_else(|| input.strip_suffix(LF))
        .unwrap_or(input)
}
