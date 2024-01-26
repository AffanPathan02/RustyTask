use std::cmp::Ordering;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,PartialEq )]
pub enum Priority{
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize)]
pub struct Task{
    id:u32,
    description:String,
    completed:bool,
    priority:Priority,
    creation_time:DateTime<Utc>,
}

pub struct TaskManager{
    pub(crate) tasks:Vec<Task>
}

impl TaskManager{
    pub fn new()->Self{
        TaskManager{tasks: Vec::new()}
    }

    pub fn add_task(&mut self, description:String,priority: Priority){
        let task=Task{
            id: self.tasks.len() as u32+1,
            description,
            completed:false,
            priority,
            creation_time:Utc::now(),
        };
        self.tasks.push(task);
        self.sort_task_by_priority();
        println!("Task Added sucessfully")
    }

    pub(crate) fn list_task(&self){
        if self.tasks.is_empty(){
            println!("No task remaining");
        }
        else{
            for task in &self.tasks{
                println!("ID:{} \nDescription:{},\nPriority:{:?} \nCompleted:{} \nCreation Time:{}",
                         task.id,task.description,task.priority,task.completed,task.creation_time);
                println!("--------XX---------XX-------")
            }
        }
    }

    pub(crate) fn mark_task_completed(&mut self, task_id:u32){
        if let Some(task)=self.tasks.iter_mut().find(|t| t.id== task_id){
            task.completed=true;
            println!("Task is marked as completed");
        }
        else {
            println!("Task marked as completed");
        }
    }

    pub(crate) fn consume_task(&mut self, priority: Priority){
        if let Some(task)=self
            .tasks
            .iter_mut()
            .filter(|t| t.priority==priority && !t.completed)
            .min_by_key(|t| match t.priority{
                Priority::High=>0,
                Priority::Medium=>1,
                Priority::Low=>3
            })
        {
            task.completed=true;
            println!("Task Consumed:{}",task.description);
        }else{
            println!("No task with specified priority available");
        }
    }

    fn sort_task_by_priority(&mut self){
        self.tasks.sort_by(|a, b| {
            match (&a.priority, &b.priority) {
                (Priority::High, Priority::High) | (Priority::Medium, Priority::Medium) | (Priority::Low, Priority::Low) => {
                    a.id.cmp(&b.id)
                }
                (Priority::High, _) => Ordering::Less,
                (Priority::Medium, Priority::High) => Ordering::Greater,
                (Priority::Medium, Priority::Low) => Ordering::Less,
                (Priority::Medium, Priority::Medium) => a.id.cmp(&b.id),
                (Priority::Low, _) => Ordering::Greater,
            }
        });

        println!("Task are sorted");
    }
}
