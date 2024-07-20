// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::UNIX_EPOCH;

// use rocksdb::{DB, Options};

use gilrs::{Gilrs, Button, Event};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    //     match db.get(b"my key") {
    //         Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
    //         Ok(None) => println!("value not found"),
    //         Err(e) => println!("operational problem encountered: {}", e),
    //     }
    //     db.delete(b"my key").unwrap();
    // }
    // let _ = DB::destroy(&Options::default(), path);

    // run gilrs in a separate thread
    let gilrs_thread = std::thread::spawn(|| {
        let mut gilrs = Gilrs::new().unwrap();
        let path = "coca-rocks.db";
        // let db = DB::open_default(path).unwrap();

        // Iterate over all connected gamepads
        for (_id, gamepad) in gilrs.gamepads() {
            println!("{} is {:?}", gamepad.name(), gamepad.power_info());
        }

        loop {
            // Examine new events
            while let Some(Event { id, event, time }) = gilrs.next_event() {
                let serialized = serde_json::to_string(&event).unwrap();

                println!("serialized = {}", serialized);
                // convert time to unix time
                let unix_time = time.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
                // db.put(unix_time.to_ne_bytes(), serialized).unwrap();
                
                // check if it is a connection event
                if event == gilrs::ev::EventType::Connected {
                    let gamepad = gilrs.gamepad(id);
                    
                    println!("connected: {:?}; power: {:?}; ff: {:?}", gamepad.name(), gamepad.power_info(), gamepad.is_ff_supported());
                }
            }
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
