// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::{atomic::Ordering, Arc}, time::{SystemTime, UNIX_EPOCH}};
use chrono::prelude::*;

use rocksdb::{DB, Options};

use gilrs::{Event, Gilrs};
use serde::{Deserialize, Serialize};

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, WindowBuilder};
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

#[derive(Serialize)]
struct Button {
    name: gilrs::Button,
    presses: i32,
}

#[derive(Serialize)]
struct Combo {
    name: String,
    pattern: Vec<String>,
    presses: i32,
}

#[derive(Serialize)]
struct AppStats {
    name: String,
    presses: Vec<Button>,
    combos: Vec<Combo>,
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
    // open default json: 3.6MiB (100k)

    // insert 1000 values of dummy data
    let pad = "PS5 Controller".to_string();
    let datas = ["{\"AxisChanged\":[\"LeftStickY\",0.010416665,{\"page\":1,\"usage\":49}]}",
        "{\"ButtonPressed\":[\"Unknown\",{\"page\":9,\"usage\":8}]}",
        "{\"ButtonPressed\":[\"DPadDown\",{\"page\":9,\"usage\":2}]}",
        "{\"ButtonPressed\":[\"DPadLeft\",{\"page\":9,\"usage\":1}]}",
        "{\"ButtonPressed\":[\"DPadRight\",{\"page\":9,\"usage\":3}]}",
        "{\"ButtonPressed\":[\"DPadUp\",{\"page\":9,\"usage\":0}]}",

        "{\"ButtonPressed\":[\"North\",{\"page\":9,\"usage\":4}]}",
        "{\"ButtonPressed\":[\"East\",{\"page\":9,\"usage\":5}]}",
        "{\"ButtonPressed\":[\"South\",{\"page\":9,\"usage\":6}]}",
        "{\"ButtonPressed\":[\"West\",{\"page\":9,\"usage\":7}]}",
    ];

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

        let data = datas[rand::random::<usize>() % datas.len()];
        let event = serde_json::from_str(data).unwrap();

        let at = unix_time + (i as u128) * t;
        let rock = Rock {
            at,
            pad: pad.clone(),
            app: app.clone(),
            event,
        };

        let serialized = bincode::serialize(&rock).unwrap();
        // let serialized = serde_json::to_string(&rock).unwrap();
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
async fn applications(timeframe: String, state: tauri::State<'_, AppState>) -> Result<Vec<Application>, ()> {
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

    Ok(apps)
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
async fn graph(timeframe: String, state: tauri::State<'_, AppState>) -> Result<Vec<Point>, ()> {
    let points = match timeframe.as_str() {
        "day" => past_time(DAY, 24, "%l %P", state),
        "week" => past_time(WEEK, 7, "%a", state),
        "month" => past_time(MONTH, 30, "%e", state), // hmmmm
        "year" => past_time(YEAR, 12, "%b", state),
        _ => past_time(DAY, 24, "%l %P", state),
    };

    Ok(points)
}

#[tauri::command]
async fn app_stats(app: String, timeframe: String, state: tauri::State<'_, AppState>) -> Result<AppStats, ()> {
    let mut app =  AppStats {
        name: app,
        presses: Vec::new(),
        combos: Vec::new(),
    };

    let span = match timeframe.as_str() {
        "day" => DAY,
        "week" => WEEK,
        "month" => MONTH,
        "year" => YEAR,
        _ => DAY,
    };
    
    let start = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() - span;

    let db = state.0.lock().unwrap().as_ref().unwrap().db.clone();

    for row in db.iterator(rocksdb::IteratorMode::End) {
        let (key, value) = row.unwrap();
        let at = u128::from_ne_bytes(key.into_vec().try_into().unwrap());

        // check if we are no longer in bounds
        if at < start {
            break;
        }

        // let value = String::from_utf8(value.into_vec()).unwrap();
        let rock: Rock = bincode::deserialize(&value.into_vec()).unwrap();
        if rock.app != app.name {
            continue;
        }

        // prec.
        // add combo
        // match other events
        // use gilrs svg
        // get app name

        // this will be auto formatted by serde when going to js
        // this really has all the events i care about
        if let gilrs::EventType::ButtonPressed(button, _code) = rock.event {
            let pressed = app.presses.iter_mut().find(|press| press.name == button);
            if let Some(pressed) = pressed {
                pressed.presses += 1;
            } else {
                app.presses.push(Button {
                    name: button,
                    presses: 1,
                });
            }
        }
    }

    Ok(app)
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

#[cfg(windows)]
unsafe extern "system" fn win_event_proc(
    _h_win_event_hook: windows::Win32::UI::WindowsAndMessaging::HWINEVENTHOOK,
    _event: u32,
    hwnd: HWND,
    _id_object: i32,
    _id_child: i32,
    _id_event_thread: u32,
    _dwms_event_time: u32,
) {
    let mut title = [0u16; 256];
    let len = windows::Win32::UI::WindowsAndMessaging::GetWindowTextW(hwnd, &mut title);
    let title = String::from_utf16_lossy(&title[..len as usize]);
    
    println!("Focus changed to: {}", title);
}

fn main() {
    let path = "coca-rocks.db";
    let mut opts = Options::default();
    opts.create_if_missing(true);
    // open default: 15.5MiB (111k)
    let db = DB::open(&opts, path).unwrap();
    let db = Arc::new(db);

    #[cfg(windows)]
    unsafe {
        let hook = SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            Some(win_event_proc),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );

        if hook.0 == 0 {
            println!("Failed to set event hook");
            return Ok(());
        }

        println!("Listening for focus changes. Press Ctrl+C to exit.");

        // Message loop
        let mut msg = std::mem::zeroed();
        while windows::Win32::UI::WindowsAndMessaging::GetMessageW(&mut msg, HWND(0), 0, 0).into() {
            windows::Win32::UI::WindowsAndMessaging::TranslateMessage(&msg);
            windows::Win32::UI::WindowsAndMessaging::DispatchMessageW(&msg);
        }
    }

    // _dummy_data(&db);
    // exit(0);

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

    let visible = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let visible_c = Arc::clone(&visible);
    let visible_c1 = Arc::clone(&visible);
    let hide = CustomMenuItem::new("toggle".to_string(), "Hide"); // i know the state
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "toggle" => {
                    let window = app.get_window("main").unwrap_or_else(|| {
                        let w = WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
                            .title("main")
                            .focused(true)
                            // .size(800.0, 600.0)
                            .build()
                            .unwrap();

                        w.hide().unwrap(); // hmm

                        w
                    });

                    let visible_v = visible_c.load(Ordering::SeqCst);
                    if visible_v {
                        window.hide().unwrap();
                        item_handle.set_title("Show").unwrap();
                    } else {
                        window.show().unwrap();
                        item_handle.set_title("Hide").unwrap();
                    }
                    visible_c.store(!visible_v, Ordering::SeqCst);
                }
                _ => {}
                }
            }
            _ => {}
        })
        .manage(AppState(std::sync::Arc::new(std::sync::Mutex::new(Some(Settings { db })))))
        .invoke_handler(tauri::generate_handler![greet, applications, graph, app_stats])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(move |app_handle, event| match event {
            // tauri::Event::Window(tauri::WindowEvent::Resized { size }) => {
            //     println!("window resized to {:?}", size);
            // }

            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();

                // set the tray icon to invisible
                let tray = app_handle.tray_handle();
                let item = tray.get_item("toggle");
                item.set_title("Show").unwrap();

                visible_c1.store(false, Ordering::SeqCst);
            }
            _ => {}
        });
}
