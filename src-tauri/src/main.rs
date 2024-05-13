// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use taskqueue::TaskQueue;
use tauri::Manager;

mod actions;
mod defaults;
mod taskqueue;

#[tokio::main]
// https://stackoverflow.com/questions/73551266/tauri-is-there-some-way-to-access-apphandler-or-window-in-regular-struct-or-sta
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle().clone();
            app.manage(TaskQueue::new(app_handle));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            actions::apollo::confirm::confirm_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

// ==========================================================================
// ==========================================================================
// ==========================================================================

// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use chromiumoxide::{error::CdpError, Browser, BrowserConfig};
// use futures::StreamExt;
// use tauri::window::Window;

// mod defaults;
// mod taskqueue;

// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let (mut browser, mut handler) =
//         Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

//     let handle = tokio::task::spawn(async move {
//         loop {
//             let _event = handler.next().await.unwrap();
//         }
//     });

//     let lol = browser
//         .start_incognito_context()
//         .await?
//         .new_page("https://www.google.com")
//         .await?;

//     let lol2 = browser
//         .start_incognito_context()
//         .await?
//         .new_page("https://www.google.com")
//         .await?;

//     let handle1 = tokio::task::spawn(async move {
//         lol.goto("https://crates.io/search?q=chromium&sort=downloads")
//             .await?
//             .goto("https://www.youtube.com")
//             .await?
//             .goto("https://dash.cloudflare.com/login")
//             .await?
//             .goto("https://profy.dev/project/react-job-simulator/welcome")
//             .await?
//             .goto("https://www.youtube.com/watch?v=Ahwoks_dawU")
//             .await?
//             .goto("https://docs.rs/chromiumoxide/0.5.0/chromiumoxide/handler/struct.Handler.html#method.try_poll_next")
//             .await?
//             .goto("https://www.game.com")
//             .await?
//             .goto("https://www.sitelike.org/similar/downduck.com/")
//             .await?;
//         Ok::<(), CdpError>(())
//     });

//     let handle2 = tokio::task::spawn(async move {
//         lol2.goto("https://www.youtube.com/watch?v=Ahwoks_dawU")
//             .await?
//             .goto("https://docs.rs/chromiumoxide/0.5.0/chromiumoxide/handler/struct.Handler.html#method.try_poll_next")
//             .await?
//             .goto("https://www.game.com")
//             .await?
//             .goto("https://www.sitelike.org/similar/downduck.com/")
//             .await?
//             .goto("https://www.youtube.com")
//             .await?
//             .goto("https://dash.cloudflare.com/login")
//             .await?
//             .goto("https://profy.dev/project/react-job-simulator/welcome")
//             .await?
//             .goto("https://www.behance.net/gallery/36090999/Liligo-Find-The-Right-Trip")
//             .await?;
//         Ok::<(), CdpError>(())
//     });

//     println!("ITS ALL IN THE GREEENEE EFSDFDGFSSWERGDSFXVZCASfgasdf");

//     tauri::Builder::default()
//         .plugin(tauri_plugin_shell::init())
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");

//     handle1.await;
//     handle2.await;
//     handle.await;

//     Ok(())
// }
