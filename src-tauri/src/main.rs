// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct TaskList {
    to_do_list: Vec<Task>
}

#[derive(Serialize, Deserialize)]
struct Task {
    task_name: String,
    finished: bool,
    is_editing: bool,
    new_task_name: String,
}



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

/*
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/

#[tauri::command]
fn create_new_template() -> TaskList {
    TaskList {
        to_do_list: vec![
            Task {
                task_name: "Do stuff".to_string(),
                finished: false,
                is_editing: false,
                new_task_name: String::new()
            }
        ]
    }
}

#[tauri::command]
fn add_task(mut file: TaskList, task_name: String) -> TaskList {
    file.to_do_list.push(Task {task_name: task_name, finished: false, is_editing: false, new_task_name: String::new()});
    file
}

#[tauri::command]
fn edit_task(mut file: TaskList, index: usize, new_task_name: String) -> Result<TaskList, String> {
    if new_task_name == String::new() {
        file.to_do_list[index].is_editing = false;
        return Ok(file);
    }
    
    if index < file.to_do_list.len() {
        file.to_do_list[index].task_name = new_task_name;
    } else {
        return Err("Index is out of bounds".to_string());
    }
    file.to_do_list[index].is_editing = false;
    file.to_do_list[index].new_task_name = String::new();
    Ok(file)
}

#[tauri::command]
fn move_up(mut file: TaskList, index: usize) -> TaskList {
    if index == 0 || index >= file.to_do_list.len() {
        file
    } else {
        file.to_do_list.swap(index, index - 1);
        file
    }

}

#[tauri::command]
fn move_down(mut file: TaskList, index: usize) -> TaskList {
    if index >= file.to_do_list.len() - 1 {
        file
    } else {
        file.to_do_list.swap(index, index + 1);
        file
    }
}

#[tauri::command]
fn delete_task(mut file: TaskList, index: usize) -> TaskList {
    file.to_do_list.remove(index as usize);
    file
}

#[tauri::command]
fn change_state(mut file: TaskList, index: usize, state: bool) -> TaskList {
    file.to_do_list[index].finished = state;
    file
}

#[tauri::command]
fn delete_all_completed_tasks(mut file: TaskList) -> TaskList {
    
    let mut i = 0;
    while i < file.to_do_list.len() {
        if file.to_do_list[i].finished {
            file.to_do_list.remove(i);
            continue;
        }
        i += 1;
    }
    
    file
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_task,
            edit_task,
            move_up,
            move_down,
            delete_task,
            change_state,
            delete_all_completed_tasks,
            create_new_template
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
