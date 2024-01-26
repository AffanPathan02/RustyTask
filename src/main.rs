mod task;

use std::fs;
use std::io::{self,Write};
use serde::{Deserialize,Serialize};
use task::{Task, Priority,TaskManager};

fn main() {
    let mut task_manager=TaskManager::new();
    loop {
        print_menu();

        let mut input=String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");

        match input.trim().parse() {
            Ok(1)=>{
                print!("Enter the task Description:");
                io::stdout()
                    .flush()
                    .unwrap();
                let mut desc=String::new();
                io::stdin()
                    .read_line(&mut desc)
                    .expect("Failed to read the input");

                println!("Select priority: ");
                println!("1. High");
                println!("2. Medium");
                println!("3. Low");
                print!("Choose priority (1-3): ");
                io::stdout().flush().unwrap();

                let mut priority_input=String::new();
                io::stdin()
                    .read_line(&mut priority_input)
                    .expect("cannot read input");

                let priority= match priority_input.trim().parse::<u32>(){
                    Ok(1)=>Priority::High,
                    Ok(2)=>Priority::Medium,
                    Ok(3)=>Priority::Low,
                    _=>{
                        println!("Invalid priority. Default it to Medium");
                        Priority::Medium
                    }
                };

                task_manager.add_task(desc.trim().to_string(),priority);
            }
            Ok(2)=>{
                task_manager.list_task();
            }
            Ok(3)=>{
                print!("Enter the task to marked as completed:");
                io::stdout()
                    .flush()
                    .unwrap();
                let mut task_id=String::new();

                io::stdin()
                    .read_line(&mut task_id)
                    .expect("Failed to read the input");

                let task_id=task_id.trim().parse::<u32>();
                if let Ok(task_id)=task_id{
                    task_manager.mark_task_completed(task_id);
                }else{
                    println!("Invalid Task id");
                }
            }
            Ok(4)=>{
                save_task(&task_manager.tasks);
                println!("Task saved to disk")
            }
            Ok(5)=>{
                load_task(&mut task_manager.tasks);
                println!("Task loaded from disk");
            }
            Ok(6)=>{
                println!("Select priority to consume:");
                println!("1. High");
                println!("2. Medium");
                println!("3. Low");
                print!("Choose priority to consume (1-3): ");
                io::stdout()
                    .flush()
                    .unwrap();

                let mut priority_input=String::new();
                io::stdin()
                    .read_line(&mut priority_input)
                    .expect("failed to read the input");

                let priority= match priority_input.trim().parse::<u32>(){
                    Ok(1)=>Priority::High,
                    Ok(2)=>Priority::Medium,
                    Ok(3)=>Priority::Low,
                    _=>{
                        println!("Invalid Priority");
                        continue;
                    }
                };

                task_manager.consume_task(priority);
            }Ok(7)=>{
                println!("Exiting the task manager");
                break;
            }
            _=>{
                println!("Invalid CLI input");
            }
        }
    }
}

fn print_menu(){
    println!("===== Task Manager Menu =====");
    println!("1. Add Task");
    println!("2. List Tasks");
    println!("3. Mark Task as Complete");
    println!("4. Save Tasks to Disk");
    println!("5. Load Tasks from Disk");
    println!("6. Consume task");
    println!("7. Exit");
    print!("Choose an option: ");
    io::stdout().flush().unwrap();
}

fn save_task(tasks: &Vec<Task>){
    let task_json=serde_json::to_string_pretty(&tasks)
        .expect("Failed to serilaize task");

    fs::write("Task.json",task_json)
        .expect("Failed to create task.json");
}

fn load_task(tasks: &mut Vec<Task>){
    if let Ok(contents)=fs::read_to_string("Task.json"){
        if let Ok(loaded_tasks)=serde_json::from_str::<Vec<Task>>(&contents){
            tasks.clear();
            tasks.extend(loaded_tasks);
        }else{
            println!("failed to load the task from disk");
        }
    }else{
        println!("No task was saved");
    }
}