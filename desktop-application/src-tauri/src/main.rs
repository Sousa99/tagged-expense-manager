// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenvy::dotenv;

use tagged_expense_manager::database::entities::expenses::Expense;
use tagged_expense_manager::services::expenses as expenses_service;

#[tauri::command]
fn get_expenses() -> Option<Vec<Expense>> {
    expenses_service::get_expenses(None).ok()
}

fn main() {
    // Load Environment Variables
    dotenv().expect("🚫 .env file could not found");
    // Initialize Logger
    env_logger::init();

    // Setup Tauri
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_expenses])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
