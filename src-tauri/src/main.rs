// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io::Read, process::exit, thread::sleep, time::{SystemTime, UNIX_EPOCH, Duration}};
use chrono::prelude::*;

use rocksdb::{DB, Options};

use gilrs::{Button, Event, Gilrs};
use serde::{Deserialize, Serialize};

use std::io::{Write, BufReader, BufRead, Error};

#[cfg(windows)]
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::System::ProcessStatus::*,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize)]
struct Application {
    name: String,
    controller: String,
    presses: i32,
    combos: i32,
}

#[derive(Serialize, Deserialize)]
struct Rock {
    at: u128,
    pad: String,
    app: String,
    event: gilrs::EventType,
}

#[derive(Serialize, Deserialize, Clone)]
struct Point {
    data: u32,
    at: u128, // used for sorting
    label: String,
}

#[tauri::command]
fn applications() -> Vec<Application> {
    let mut apps = Vec::<Application>::new();
    
    // open an iter to read the db
    let path = "coca-rocks.db";
    let opts = Options::default();
    let db = DB::open(&opts, path).unwrap();

    let iter = db.iterator(rocksdb::IteratorMode::Start);
    for row in iter {
        let (key, value) = row.unwrap();
        let value = String::from_utf8(value.into_vec()).unwrap();

        let rock: Rock = serde_json::from_str(&value).unwrap();

        let app = apps.iter_mut().find(|app| app.name == rock.app);
        if let Some(app) = app {
            app.presses += 1;
        } else {
            apps.push(Application {
                name: rock.app,
                controller: rock.pad,
                presses: 1,
                combos: 0,
            });
        }
    }

    apps
}

const SECOND: u128 = 1_000;
const MINUTE: u128 = 60 * SECOND; // 60_000 ms
const HOUR: u128 = 60 * MINUTE; // 3_600_000 ms
const DAY: u128 = 24 * HOUR; // 86_400_000 ms
const WEEK: u128 = 7 * DAY; // 604_800_000 ms
const MONTH: u128 = 30 * DAY; // 2_592_000_000 ms
const YEAR: u128 = 365 * DAY; // 31_536_000_000 ms


fn past_day() -> Vec<Point> {
    let unix_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
    // get the time 24 hours ago
    let unix_time_24 = unix_time - DAY;

    // could do a vec, but would need one for each day anyway.
    // this would be better for adding hours with no data
    let mut buckets: [Point; 24] = std::array::from_fn(|i| {
        let at = unix_time_24 + (i as u128 * HOUR);

        // Formats the combined date and time with the specified format string.
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(at as u64);
        let label = DateTime::<Utc>::from(time).format("%l %P").to_string();

        Point { 
            data: 0, label, at
        }
    });

    // open an iter to read the db
    let path = "coca-rocks.db";
    let opts = Options::default();
    let db = DB::open(&opts, path).unwrap();

    for row in db.iterator(rocksdb::IteratorMode::Start) {
        let (key, _value) = row.unwrap();
        let at = u128::from_ne_bytes(key.into_vec().try_into().unwrap());

        // add the data to the proper bucket
        for i in 0..24 {
            // this will deal with oob
            if at >= buckets[i].at && at < buckets[i].at + HOUR {
                buckets[i].data += 1;
                break;
            }
        }
    }

    buckets.to_vec()
}

fn past_week() -> Vec<Point> {
    let unix_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
    // get the time 24 hours ago
    let unix_time_7 = unix_time - WEEK;

    // could do a vec, but would need one for each day anyway.
    // this would be better for adding hours with no data
    let mut buckets: [Point; 24] = std::array::from_fn(|i| {
        let at = unix_time_7 + (i as u128 * DAY);

        // Formats the combined date and time with the specified format string.
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(at as u64);
        let label = DateTime::<Utc>::from(time).format("%a").to_string();

        Point { 
            data: 0, label, at
        }
    });

    // open an iter to read the db
    let path = "coca-rocks.db";
    let opts = Options::default();
    let db = DB::open(&opts, path).unwrap();

    for row in db.iterator(rocksdb::IteratorMode::Start) {
        let (key, _value) = row.unwrap();
        let at = u128::from_ne_bytes(key.into_vec().try_into().unwrap());

        // add the data to the proper bucket
        for i in 0..7 {
            // this will deal with oob
            if at >= buckets[i].at && at < buckets[i].at + DAY {
                buckets[i].data += 1;
                break;
            }
        }
    }

    buckets.to_vec()
}

#[tauri::command]
fn graph(timeframe: String) -> Vec<Point> {
    let points = match timeframe.as_str() {
        "day" => past_day(),
        "week" => past_week(),
        // "month" => past_month(),
        // "year" => past_year(),
        _ => past_day(),
    };

    points
}

#[cfg(windows)]
fn get_foreground_process() -> Result<String> {
    unsafe {
        let window = GetForegroundWindow();
        if window.0 == 0 {
            return Err(Error::from_win32());
        }

        let mut process_id = 0;
        GetWindowThreadProcessId(window, Some(&mut process_id));

        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, process_id)?;

        let mut buffer = [0u16; 260];
        let size = GetModuleFileNameExW(process_handle, None, &mut buffer);
        
        if size == 0 {
            return Err(Error::from_win32());
        }

        let process_name = String::from_utf16_lossy(&buffer[..size as usize]);
        Ok(process_name)
    }
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
    // {
    //     let path = "coca-rocks.db";
    //     let mut opts = Options::default();
    //     opts.create_if_missing(true);
    //     // open default: 15.5MiB (111k)
    //     let db = DB::open(&opts, path).unwrap();

    //     // insert 1000 values of dummy data
    //     let pad = "PS5 Controller".to_string();
    //     let data = "{\"AxisChanged\":[\"LeftStickY\",0.010416665,{\"page\":1,\"usage\":49}]}";
    //     let event = serde_json::from_str(data).unwrap();

    //     let start = SystemTime::now();
    //     let n = 100000;
    //     let t = 100;
    //     let unix_time = start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() - (n as u128) * t;
    //     for i in 0..n {
    //         let app = match rand::random::<u32>() % 5 {
    //             0 => "Skyrim".to_string(),
    //             1 => "Minecraft".to_string(),
    //             2 => "Hatsune Miku Project Diva 2nd Stage".to_string(),
    //             3 => "Muse Dash".to_string(),
    //             4 => "Tekken 8".to_string(),
    //             _ => "?".to_string(),
    //         };

    //         let at = unix_time + (i as u128) * t;
    //         let rock = Rock {
    //             at,
    //             pad: pad.clone(),
    //             app: app.clone(),
    //             event,
    //         };

    //         let serialized = serde_json::to_string(&rock).unwrap();
    //         // i think doing 100_000 with time::now is too fast
    //         // somehow, using the same key gives more than one row
    //         db.put(at.to_ne_bytes(), serialized).unwrap();
    //         // sleep(Duration::from_millis(100)); // wont do anything besides slow it down. im using unix_time as the key
    //         // db.flush().unwrap(); // this will make the db big, but has no data
    //     }
    //     db.flush().unwrap();
    //     let end = SystemTime::now();
    //     println!("inserted {n} values in {:?}", end.duration_since(start).unwrap());
    //     // exit(0);
    // }

    // run gilrs in a separate thread
    let gilrs_thread = std::thread::spawn(|| {
    let mut gilrs = Gilrs::new().unwrap();
    let path = "coca-rocks.db";
    // let mut opts = Options::default();
    // opts.create_if_missing(true);
    // // open default: 15.5MiB (111k)
    // let db = DB::open(&opts, path).unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut pad = "?".to_string();
    let mut app = "?".to_string();

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            // check if it is a connection event
            if event == gilrs::ev::EventType::Connected {
                let gamepad = gilrs.gamepad(id);
                pad = gamepad.name().to_string();
                
                println!("connected: {:?}; power: {:?}; ff: {:?}", pad, gamepad.power_info(), gamepad.is_ff_supported());
            }

            let unix_time = time.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
            let rock = Rock {
                at: unix_time,
                pad: pad.clone(),
                app: app.clone(),
                event,
            };

            let serialized = serde_json::to_string(&rock).unwrap();

            println!("serialized = {}", serialized);

            // db.put(unix_time.to_ne_bytes(), serialized).unwrap();
        }
    }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, applications, graph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
