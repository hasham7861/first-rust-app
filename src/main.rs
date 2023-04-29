use std::io::{self, Write};
use std::process::{Command};

const CMD_NAME: &str = if cfg!(target_os = "windows") { "cmd" } else { "bash" };

fn main() -> io::Result<()> {
    // List available devices in pairing mode
    let list_cmd = match std::env::consts::OS {
        "windows" => format!("rundll32.exe irprops.cpl"),
        "linux" => format!("bluetoothctl paired-devices"),
        "macos" => format!("blueutil --paired"),
        _ => panic!("Unsupported OS!"),
    };
    let output = Command::new(CMD_NAME)
        .args(list_cmd.split_whitespace())
        .output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    // Ask the user to select a device
    let mut input = String::new();
    println!("\nEnter the name or MAC address of the device you want to connect to: ");
    io::stdin().read_line(&mut input)?;
    let device = input.trim();

    // Pair the selected device
    let pair_cmd = match std::env::consts::OS {
        "windows" => format!("rundll32.exe irprops.cpl,{} {}", "BluetoothAuthenticationAgent", device),
        "linux" => format!("bluetoothctl pair {}", device),
        "macos" => format!("blueutil --pair {}", device),
        _ => panic!("Unsupported OS!"),
    };
    let output = Command::new(CMD_NAME)
        .args(pair_cmd.split_whitespace())
        .output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    // Connect to the selected device
    let connect_cmd = match std::env::consts::OS {
        "windows" => format!("rundll32.exe irprops.cpl,{} {}", "BluetoothAuthenticationAgent", device),
        "linux" => format!("bluetoothctl connect {}", device),
        "macos" => format!("blueutil --connect {}", device),
        _ => panic!("Unsupported OS!"),
    };
    let output = Command::new(CMD_NAME)
        .args(connect_cmd.split_whitespace())
        .output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(())
}
