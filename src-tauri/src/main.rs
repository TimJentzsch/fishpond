#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, BufWriter, Write},
    process::{Child, Command, Stdio},
    sync::Mutex,
};

use tauri::State;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_engine])
        .manage(EngineProcesses {
            processes: Default::default(),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct EngineProcesses {
    processes: Mutex<HashMap<String, Child>>,
}

#[tauri::command]
fn run_engine(state: State<EngineProcesses>) {
    println!("Running engine!");

    let cmd = Command::new("stockfish")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    state
        .processes
        .lock()
        .unwrap()
        .insert("stockfish".to_string(), cmd);

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
