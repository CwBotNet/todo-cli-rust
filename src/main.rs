use chrono::Local;
use csv::{ReaderBuilder, Writer};
use std::io::Write;
use std::{fs::File, io::Read};
use std::{io, process};

extern crate prettytable;
use prettytable::{Cell, Row, Table};

#[derive(Debug)]
struct Task {
    title: String,
    is_done: bool,
    created_at: String,
    deadline: Option<String>,
}

fn main() {
    println!("Welcome to cli todo application build by Raj...");
    println!("commands: add / list task / update / remove / help & ?");

    let mut tasks: Vec<Task> = Vec::new();

    fn clear_terminal() {
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", "cls"])
                .status()
                .unwrap();
        }

        #[cfg(not(target_os = "windows"))]
        {
            std::process::Command::new("clear").status().unwrap();
        }
    }

    loop {
        print!("> "); // Show prompt
        io::stdout().flush().unwrap(); // Ensure it appears immediatelyß
        let mut user_command: String = String::new();

        io::stdin()
            .read_line(&mut user_command)
            .expect("unable to read the line");

        match user_command.trim().to_lowercase().as_str() {
            "add" => {
                let task = add_task();
                tasks.push(task);
                println!("Task added ✅");
            }
            "list task" => {
                list_task(&tasks);
            }
            "update" => {
                update_task(&mut tasks);
            }
            "remove" => {
                remove_task(&mut tasks);
            }
            "save" => {
                save_tasks_to_file(&tasks);
                println!("tasks saved ✅")
            }
            "help" | "?" => {
                println!(
                    "commnad list: \n 
                    add : add task \n 
                    list task : show all tasks \n
                    update : toggle a single task \n
                    remove : delete a particular task by id \n
                    save : save task to a csv file \n
                    load tasks : load tasks from csv files \n
                    exit : exit form cli app \n
                    help / ? : show command list"
                )
            }
            "exit" => process::exit(0),
            "clear" | "cls" => {
                clear_terminal();
            }
            "load tasks" => {
                tasks = load_tasks_from_file();
            }
            _ => {
                println!("invalid Command !!!")
            }
        }
    }
}
fn add_task() -> Task {
    // Read title
    println!("enter your task:");

    let mut title = String::new();

    io::stdin()
        .read_line(&mut title)
        .expect("faild to read line");
    // Read optioanl Deadline
    println!("Enter deadline (or press enter to skip):");
    let mut dl_input = String::new();
    io::stdin().read_line(&mut dl_input).unwrap();
    let deadline = if dl_input.trim().is_empty() {
        None
    } else {
        Some(dl_input.trim().to_string())
    };

    // Timestamp
    let created_at = Local::now().to_string();

    Task {
        title,
        is_done: false,
        created_at,
        deadline,
    }
}

fn list_task(tasks: &Vec<Task>) {
    let mut table = Table::new();

    table.set_titles(Row::new(vec![
        Cell::new("#"),
        Cell::new("Title"),
        Cell::new("Created At"),
        Cell::new("Deadline"),
        Cell::new("Status"),
    ]));

    // Task rows

    for (i, task) in tasks.iter().enumerate() {
        table.add_row(Row::new(vec![
            Cell::new(&format!("{}", i + 1)),
            Cell::new(&task.title.trim()),
            Cell::new(&task.created_at),
            Cell::new(task.deadline.as_deref().unwrap_or("none")),
            Cell::new(if task.is_done { "✅" } else { "❌" }),
        ]));
    }

    table.printstd();
}

fn update_task(tasks: &mut Vec<Task>) {
    // Step 1: Show task list
    list_task(tasks);
    // Step 2: Ask user for task number
    println!("Enter the task number You want to update: ");
    let mut task_id = String::new();
    io::stdin().read_line(&mut task_id).unwrap();
    let index: usize = task_id.trim().parse().unwrap();
    // Step 3: Use get_mut(index) to mark it done
    if let Some(task) = tasks.get_mut(index - 1) {
        task.is_done = true;
        println!("✅ Task marked as done!");
    } else {
        println!("❌ Task not Found");
    }
}

fn remove_task(tasks: &mut Vec<Task>) {
    // 1. Show tasks
    list_task(tasks);
    // 2. Ask for index
    println!("Enter the task id to remove: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // 3. Validate index
    let index: usize = match input.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ please Enter a Valid number!!!");
            return;
        }
    };
    // 4. Remove or show error
    if index > 0 && index <= tasks.len() {
        tasks.remove(index - 1);
        println!("🗑 task remove successfully!");
    } else {
        println!("❌ invalid task number.")
    }
}

// save task to csv file function
fn save_tasks_to_file(tasks: &Vec<Task>) {
    let file = File::create("tasks.csv").expect("unable to create file");
    let mut wtr = Writer::from_writer(file);

    for task in tasks {
        wtr.write_record(&[
            &task.title.trim().replace("\n", ""),
            &task.created_at,
            &task.deadline.clone().unwrap_or_default(),
            &task.is_done.to_string(),
        ])
        .expect("unable to write task")
    }

    wtr.flush().expect("failed to save tasks")
}

fn load_tasks_from_file() -> Vec<Task> {
    let mut tasks: Vec<Task> = Vec::new();
    // Try to open "tasks.csv"
    println!("Enter the csv task file path: ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let path = input.trim();

    // debug
    let mut file_str = String::new();
    File::open(path)
        .unwrap()
        .read_to_string(&mut file_str)
        .unwrap();
    println!("🗂 file contents:\n{}", file_str);

    let file = match File::open(path) {
        Ok(f) => f,
        // If it fails (e.g. file not found), return an empty Vec
        Err(_) => return tasks,
    };
    println!("{}", path);
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);
    // Otherwise, read each record and map it into a Task

    for result in rdr.records() {
        let record = result.expect("faild to read CSV row");
        println!("Parsed Record: {:?}", record);
        let task = Task {
            title: record[0].to_string(),
            created_at: record[1].to_string(),
            deadline: {
                let d = record.get(2).unwrap_or("").trim();
                if d.is_empty() {
                    None
                } else {
                    Some(d.to_string())
                }
            },
            is_done: record
                .get(3)
                .unwrap_or("false")
                .parse::<bool>()
                .unwrap_or(false),
        };

        tasks.push(task);
    }

    tasks
}
