#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::api::dialog;
use std::path::Path;
use cmd::*;

mod cmd;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => {
          Err(e.to_string())
        }
        Ok(command) => {
          match command {
            cmd::Cmd::Login(UserLogin) => {},
            cmd::Cmd::Register(UserRegister) => {},
            cmd::Cmd::GetUserData(uid) => {},
            cmd::Cmd::ChooseFolder => {},
            cmd::Cmd::ClickedBtn => {},
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}


