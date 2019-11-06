use std::str;
use std::io::{Write};
use std::process::{Command, Stdio};


fn main() {


    let output = Command::new("ratpoison").arg("-c").arg("windows %n,%c,%l,%s,%a,%t").output().expect("Failed to retrieve list of windows from Ratpoison");
    let output = String::from_utf8_lossy(output.stdout.as_slice());

    let dmenu_args = ["-i", "-l", "3"];
    let mut ext_process = Command::new("dmenu").args(&dmenu_args).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Error opening dmenu");

    let ext_process_stdin = ext_process.stdin.as_mut().unwrap();

    for line in output.lines() {
        let line_ln = format!("{}\n", line);
        ext_process_stdin.write_all(line_ln.as_bytes()).expect("Error sending list of windows to dmenu");
    }

    let output = ext_process.wait_with_output().expect("Error while getting chosen window form dmenu");
    let chosen_window = str::from_utf8(&output.stdout).unwrap().trim();
    // println!("{}", chosen_window);

    let mut fields = chosen_window.split(',');
    let window_number = fields.next();
    match window_number {
        Some(num) => {
            let rp_command = format!("select {}", num);
            Command::new("ratpoison").arg("-c").arg(rp_command).output().expect("Failed to switch windows in Ratpoison");
        },
        None => (), //No window chosen
    }
}
