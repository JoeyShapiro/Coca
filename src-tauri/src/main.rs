// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use chrono::prelude::*;

use rocksdb::{DB, Options};

use gilrs::{Event, Gilrs};
use serde::{Deserialize, Serialize};

#[cfg(windows)]
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::System::ProcessStatus::*,
};

struct Settings {
    db: Arc<DB>,
}

#[derive(Default)]
struct AppState(std::sync::Arc<std::sync::Mutex<Option<Settings>>>);

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

#[derive(Serialize, Deserialize, Debug)]
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

fn _dummy_data(db: &DB) {
    // open default json bad-id: 15.5MiB (111k)
    // open default bin: 2.7MiB (>100k)

    // insert 1000 values of dummy data
    let pad = "PS5 Controller".to_string();
    let data = "{\"AxisChanged\":[\"LeftStickY\",0.010416665,{\"page\":1,\"usage\":49}]}";
    let event = serde_json::from_str(data).unwrap();

    let start = SystemTime::now();
    let n = 100000;
    let t = 100;
    let unix_time = start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() - (n as u128) * t;
    for i in 0..n {
        let app = match rand::random::<u32>() % 5 {
            0 => "Skyrim".to_string(),
            1 => "Minecraft".to_string(),
            2 => "Hatsune Miku Project Diva 2nd Stage".to_string(),
            3 => "Muse Dash".to_string(),
            4 => "Tekken 8".to_string(),
            _ => "?".to_string(),
        };

        let at = unix_time + (i as u128) * t;
        let rock = Rock {
            at,
            pad: pad.clone(),
            app: app.clone(),
            event,
        };

        let serialized = bincode::serialize(&rock).unwrap();
        // i think doing 100_000 with time::now is too fast
        // somehow, using the same key gives more than one row
        db.put(at.to_ne_bytes(), serialized).unwrap();
        // sleep(Duration::from_millis(100)); // wont do anything besides slow it down. im using unix_time as the key
        // db.flush().unwrap(); // this will make the db big, but has no data
    }
    db.flush().unwrap();
    let end = SystemTime::now();
    println!("inserted {n} values in {:?}", end.duration_since(start).unwrap());
}

#[tauri::command]
fn applications(timeframe: String, state: tauri::State<'_, AppState>) -> Vec<Application> {
    let mut apps = Vec::<Application>::new();

    let span = match timeframe.as_str() {
        "day" => DAY,
        "week" => WEEK,
        "month" => MONTH,
        "year" => YEAR,
        _ => DAY,
    };
    let start = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() - span;
    
    let db = state.0.lock().unwrap().as_ref().unwrap().db.clone();

    let iter = db.iterator(rocksdb::IteratorMode::End);
    for row in iter {
        let (key, value) = row.unwrap();

        // skip if to old
        let at = u128::from_ne_bytes(key.into_vec().try_into().unwrap());
        if at < start {
            break; // we are reversed, so we can break
        }

        // let value = String::from_utf8(value.into_vec()).unwrap();
        let rock: Rock = bincode::deserialize(&value.into_vec()).unwrap();

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

fn past_time(span: u128, n: u128, form: &str, state: tauri::State<'_, AppState>) -> Vec<Point> {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() - span;

    // create the buckets
    let mut buckets: Vec<Point> = Vec::with_capacity(n as usize);
    for i in 0..n {
        let at = start + (i as u128 * (span / n));

        // Formats the combined date and time with the specified format string.
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(at as u64);
        let label = DateTime::<Utc>::from(time).format(form).to_string();

        buckets.push(Point { 
            data: 0, label, at
        })
    }

    let db = state.0.lock().unwrap().as_ref().unwrap().db.clone();

    for row in db.iterator(rocksdb::IteratorMode::End) {
        let (key, _value) = row.unwrap();
        let at = u128::from_ne_bytes(key.into_vec().try_into().unwrap());

        // add the data to the proper bucket
        for i in 0..n as usize {
            // this will deal with oob
            if at >= buckets[i].at && at < buckets[i].at + (span / n) {
                buckets[i].data += 1;
                break;
            }
        }

        // check if we are no longer in bounds
        if at < start {
            break;
        }
    }

    buckets
}

#[tauri::command]
fn graph(timeframe: String, state: tauri::State<'_, AppState>) -> Vec<Point> {
    let points = match timeframe.as_str() {
        "day" => past_time(DAY, 24, "%l %P", state),
        "week" => past_time(WEEK, 7, "%a", state),
        "month" => past_time(MONTH, 30, "%e", state), // hmmmm
        "year" => past_time(YEAR, 12, "%b", state),
        _ => past_time(DAY, 24, "%l %P", state),
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
    let path = "coca-rocks.db";
    let mut opts = Options::default();
    opts.create_if_missing(true);
    // open default: 15.5MiB (111k)
    let db = DB::open(&opts, path).unwrap();
    let db = Arc::new(db);

    // run gilrs in a separate thread
    let db_put = Arc::clone(&db);
    let _gilrs_thread = std::thread::spawn(move || {
        let mut gilrs = Gilrs::new().unwrap();

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

                let serialized = bincode::serialize(&rock).unwrap();

                // println!("serialized = {:?}", serialized);

                db_put.put(unix_time.to_ne_bytes(), serialized).unwrap();
                println!("{rock:?}");
            }
        }
    });

    tauri::Builder::default()
        .manage(AppState(std::sync::Arc::new(std::sync::Mutex::new(Some(Settings { db })))))
        .invoke_handler(tauri::generate_handler![greet, applications, graph])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
