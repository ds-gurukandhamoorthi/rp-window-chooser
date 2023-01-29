use std::str;
use std::io::{Write};
use std::process::{Command, Stdio};


fn main() {
    let tmux_window_info_format = "tmux,#{window_index},#{window_name}#{window_flags},(#{window_panes} panes)";
    let tmux_list_windows_args = ["list-windows", "-F", tmux_window_info_format];
    let output_tmux = Command::new("tmux").args(tmux_list_windows_args).output().expect("Failed to retrieve list of windows from Tmux");
    let output_tmux = String::from_utf8_lossy(output_tmux.stdout.as_slice());


    let output = Command::new("ratpoison").arg("-c").arg("windows %n,%c,%l,%s,%a,%t").output().expect("Failed to retrieve list of windows from Ratpoison");
    let output = String::from_utf8_lossy(output.stdout.as_slice());

    let dmenu_args = ["-i", "-l", "3"];
    let mut ext_process = Command::new("dmenu").args(dmenu_args).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Error opening dmenu");

    let ext_process_stdin = ext_process.stdin.as_mut().unwrap();

    let window_list = output_tmux.lines().chain(output.lines());
    for line in window_list {
        let line_ln = format!("{line}\n");
        ext_process_stdin.write_all(line_ln.as_bytes()).expect("Error sending list of tmux windows to dmenu");
    }

    let output = ext_process.wait_with_output().expect("Error while getting chosen window form dmenu");
    let chosen_window = str::from_utf8(&output.stdout).unwrap().trim();

    //when we have not chosen anything in dmenu, the resulting string's length is 0  (after it is trimmed)
    if !chosen_window.is_empty() {
        let mut fields = chosen_window.split(',');
        if chosen_window.starts_with("tmux") {
            let window_number = fields.nth(1).unwrap();
            let tmux_args = ["select-window", "-t", window_number];
            Command::new("tmux").args(tmux_args).output().expect("Failed to switch windows in Tmux");

        } else {

            let window_number = fields.next();

            if let Some(num) = window_number {
                let num: i32 = num.parse().expect("The window number was not an integer"); //this will help us avoid any error in our formatting string "%n,%c".. Had we written "%n|%c" there won't be a number hwn split by ',' and we would catch the error here.
                let rp_command = format!("select {num}");
                Command::new("ratpoison").arg("-c").arg(rp_command).output().expect("Failed to switch windows in Ratpoison");
            }
        }
    }
}
