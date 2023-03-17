use chrono::NaiveDate;
mod date_functions;
mod formatting;
mod task_file;
use std::env;
const TO_DO_FILE: &str = "/home/sherlly/.local/share/todo_file.csv";

struct Task {
    task_title: String,
    task_date: NaiveDate
}



fn main() {
let args: Vec<String> = env::args().collect();
let mode = &args[1];

if mode == "--upcoming" {
//silence for notifications
formatting::formatter();
}
else {
 println!("\n setting to do file in {} \n change in source code",&TO_DO_FILE);
 println!("\n Welcome to Sherlly's task manager (Taskr) \n use flag -h for help" );


if mode == "-h" || mode == "--help" {
    println!("\n
SET WHERE THE TODO FILE SHOULD GO IN THE SOURCE CODE BEFORE COMPILING \n

--list will show a list of tasks \n
--add will add a task \n
--done will will delete a task \n
--upcoming will give a count of tasks due today/tomorrow and within the next 7 days \n
--clear will delete all tasks \n

             ");
}

if mode == "--list" {
task_file::list_all();
}


else if mode == "--clear" {
task_file::clear();
println!("\n cleared task list \n")
}

else if mode == "--add" {

let task_title = args[2].parse::<String>().unwrap();
let task_date = args[3].parse::<String>().unwrap();
task_file::add_task(&task_title,&task_date);
println!("added task!");

}


else if mode == "--done" {
let task_title = args[2].parse::<String>().unwrap();
task_file::remove_task(&task_title);
println!("Task removed!");

}
else if mode == "--import_ics" {
let file_path = args[2].parse::<String>().unwrap();
task_file::import_ics(&file_path);
println!("Imported from ics file!");

}
else {
    println!("please type in a mode in the format taskr --mode <options> where the modes are: upcoming, done, add, list, clear " );


}
}

}
