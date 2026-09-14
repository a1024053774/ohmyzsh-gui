mod config;
mod engine;
mod github;
mod system;

use engine::{Manager, Request, Response};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
fn dispatch(request: Request, state: State<'_, Mutex<Manager>>) -> Result<Response, String> {
    state
        .lock()
        .map_err(|_| "Application state is unavailable".to_string())?
        .request(request)
}
fn main() {
    let manager = Manager::new().expect("unable to initialize ohmyzsh-gui");
    tauri::Builder::default()
        .manage(Mutex::new(manager))
        .invoke_handler(tauri::generate_handler![dispatch])
        .run(tauri::generate_context!())
        .expect("error while running ohmyzsh-gui");
}
