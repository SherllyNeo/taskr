extern crate csv;
use std::fs::File;
use crate::TO_DO_FILE;
use crate::Task;
use crate::date_functions::{string_to_date,difference_between_dates};



pub fn formatter() {

    let mut really_urgent: i32 = 0;
    let mut urgent: i32 = 0;
    let mut soon: i32 = 0;
    let file_read = File::open(TO_DO_FILE).expect("could not open file path");

    let mut rdr = csv::Reader::from_reader(file_read);


    let mut to_do_list: Vec<Task> = Vec::new();

    for result in rdr.records() {
        let record = result.expect("could not open dns record");
        let task_title = &record[0];
        let task_date = &record[1];
        let dt = string_to_date(task_date);
        let new_task = Task { task_title: task_title.to_owned(), task_date: dt };
        to_do_list.push(new_task);
    }
    for task in to_do_list {
        let today_nav = chrono::offset::Local::now().date_naive();
        let diff = difference_between_dates(task.task_date,today_nav);
        if diff<0 {
            really_urgent+=1;
        }
        else if diff<2 {
            urgent+=1;
        }
        else if diff <=7 {
            soon+=1;
        }
        else {
        }

    }
    println!("|❌{} ❗{}  📅{} |",really_urgent,urgent,soon);




}
