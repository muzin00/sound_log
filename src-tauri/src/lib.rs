mod commands;
mod player;
mod record;
mod recorder;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut record = record::Record::new();
    let recorder = recorder::Recorder::new(&mut record);
    let player = player::Player::new(&record);

    tauri::Builder::default()
        .manage(recorder)
        .manage(player)
        .invoke_handler(tauri::generate_handler![
            commands::start_recording,
            commands::stop_recording,
            commands::play_audio,
            commands::stop_audio,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
