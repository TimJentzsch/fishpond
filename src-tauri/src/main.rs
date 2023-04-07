#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    process::{Command, Stdio},
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_engine])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn run_engine() {
    println!("Running engine!");

    let cmd = Command::new("stockfish")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(std_in) = cmd.stdin {
        let mut writer = BufWriter::new(std_in);

        writer.write_all("uci".as_bytes()).unwrap();
    }

    if let Some(std_out) = cmd.stdout {
        let mut reader = BufReader::new(std_out);

        let mut line = String::new();

        while let Ok(bytes) = reader.read_line(&mut line) {
            if bytes == 0 {
                break;
            }

            println!("Engine: {line}");
        }
    }
}
