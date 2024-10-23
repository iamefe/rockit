// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod db_connect;
mod task;

use db_connect::setup_database;
use task::Task;

#[tauri::command(rename_all = "snake_case")]
pub fn create_task(description: String) -> Result<String, String> {
    println!("Adding task with lib.rs");
    // `map_err(|e| e.to_string())?` When exposing functions to Tauri commands, you need to return Result types with String for errors so that they can be easily displayed or logged by the frontend.
    let conn = setup_database().map_err(|e| e.to_string())?;
    Task::add_task(&conn, &description).map_err(|e| e.to_string())?;
    println!("Task added successfully");
    Ok(format!("Task '{}' added successfully!", description))
}

#[tauri::command]
pub fn fetch_tasks() -> Result<Vec<Task>, String> {
    println!("Getting tasks with lib.rs");
    let conn = setup_database().map_err(|e| e.to_string())?;
    Ok(Task::get_tasks(&conn).map_err(|e| e.to_string())?)
}

#[tauri::command(rename_all = "snake_case")]
pub fn fetch_task_by_id(task_id: u32) -> Result<Option<Task>, String> {
    println!("Finding a task with lib.rs");
    let conn = setup_database().map_err(|e| e.to_string())?;
    Ok(Task::get_task_by_id(&conn, task_id).map_err(|e| e.to_string())?)
}

#[tauri::command(rename_all = "snake_case")]
pub fn edit_task(
    task_id: u32,
    new_description: Option<String>,
    completed: Option<bool>,
) -> Result<String, String> {
    println!("Updating task with lib.rs");
    let conn = setup_database().map_err(|e| {
        eprintln!("Database connection error: {}", e);
        e.to_string()
    })?;
    println!("Connected successfully");
    Task::update_task(&conn, task_id, new_description.as_deref(), completed).map_err(|e| {
        eprintln!("Error updating task: {}", e);
        e.to_string()
    })?;
    println!("Task updated successfully");
    Ok(format!("Task '{}' updated successfully!", task_id))
}

#[tauri::command(rename_all = "snake_case")]
pub fn remove_task(task_id: u32) -> Result<String, String> {
    let conn = setup_database().map_err(|e| e.to_string())?;
    Task::delete_task(&conn, task_id).map_err(|e| e.to_string())?;
    Ok(format!("Task '{}' deleted successfully", task_id))
}
