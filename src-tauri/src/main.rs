#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs::File, io::Read};

#[tauri::command]
fn send_memory_content(from: usize, to: usize) -> String {
    println!("::send mem");
    let mut file = File::open("../screen_2.bin").unwrap();
    let mut ram: Vec<u8> = vec![];
    file.read_to_end(&mut ram).unwrap();
    hex_format(&ram, from, to)
}

fn hex_format(buffer: &Vec<u8>, from: usize, to: usize) -> String {
    let mut res = String::new();

    let begin = (from / 16) * 16;
    let num_line = if (to - begin) % 16 == 0 {
        (to - begin) / 16
    } else {
        (to - begin) / 16 + 1
    };

    for i in 0..num_line {
        res.push_str(format!("{:04x}:", begin + i*16).as_str());
        for j in 0..16 {
            res.push_str(format!(" {:02x}", buffer[begin + i*16 + j]).as_str());
        }
        res.push('\n');
    }
    res
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_memory_content])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
