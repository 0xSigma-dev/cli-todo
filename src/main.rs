use serde::{Serialize, Deserialize};
use std::fs;
use std::env;
use uuid::Uuid;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: String,
    description: String,
    completed: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct ToDoList {
    tasks: Vec<Task>
}

impl ToDoList {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn add_tasks(&mut self, description: String) {
        let num = Uuid::new_v4().to_string();
        self.tasks.push(Task {id: num, description, completed: false});
    }

    fn list_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed {"[✔]"} else {"[ ]"};
            println!("{} {} {} - {}", index+1, task.id, status, task.description)
        }
    }

    fn complete_task(&mut self, id: String) -> bool {
        for task in self.tasks.iter_mut() {
            if task.id == id {
                task.completed = true;
                return true;
            }
        }
        false
    }

    fn remove_task(&mut self, id: String){
        for (index,task) in self.tasks.iter().enumerate() {
            if task.id == id {
                self.tasks.remove(index);
              return
        }
    } 
    }

    fn save_to_file(&self, filename: &str) {
        let data = serde_json::to_string_pretty(self).expect("Cannot Serialize data");
        fs::write(filename, data).expect("Cannot write data to file");
    }

    fn load_from_file(filename: &str) -> Self {
        if let Ok(data) = fs::read_to_string(filename) {
            serde_json::from_str(&data).unwrap_or_else(|_| ToDoList::new())
        } else {
            ToDoList::new()
        }
    }
}

fn main() {
    let filename="task.json";
    let mut todo_list = ToDoList::load_from_file(filename);
    let args: Vec<String> = env::args().collect();


    if args.len() < 2 {
        println!("Usage:");
        println!("  cli-todo add \"Task description\"");
        println!("  cli-todo list");
        println!("  cli-todo complete <task_number>");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please enter a description")
            } else {
                let description = args[2..].join("");
                todo_list.add_tasks(description);
                todo_list.save_to_file(filename);
                println!("Task added successfully")
            }

        },
        "list" => {
            todo_list.list_tasks();
        },
        "complete" => {
            if args.len() < 3 {
                println!("Please provide an Id")
            } else {
                let id = args[2].to_string();
                todo_list.complete_task(id);
                todo_list.save_to_file(filename);
                println!("Task completed Successfully")
            }

        },
        "remove" => {
            if args.len() < 3 {
                println!("please provide an Id")
            } else {
                let id = args[2].to_string();
                todo_list.remove_task(id);
                todo_list.save_to_file(filename);
                println!("Task removed Successfully")
            }
        }
        _ => {
            println!("Unknown command: {}", args[1]);
        }

    }
    
}
