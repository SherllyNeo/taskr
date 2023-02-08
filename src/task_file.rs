extern crate csv;
use std::fs;
use std::fs::File;
use crate::TO_DO_FILE;
use std::fs::OpenOptions;
use crate::Task;
use crate::date_functions::string_to_date;
use crate::date_functions::date_to_string;
use chrono::NaiveDate;


pub fn add_task(task_title: &str, task_date: &str ) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(TO_DO_FILE)
        .unwrap();
    let mut wtr = csv::Writer::from_writer(file);

    wtr.write_record(&[task_title, task_date]).expect("could not write to to do list");
    wtr.flush().expect("Failed to flush");
}
pub fn clear() {

    fs::remove_file(TO_DO_FILE).expect("unable to remove to do list file");

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(TO_DO_FILE)
        .unwrap();
    let mut wtr = csv::Writer::from_writer(file);

    wtr.write_record(&["task", "date"]).expect("could not write to to do list");

    wtr.flush().expect("Failed to flush");
}

pub fn remove_task(task_title: &str) {
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
    let filtered_to_do_list: Vec<Task> = to_do_list.into_iter().filter(|task| task.task_title != task_title).collect();


    fs::remove_file(TO_DO_FILE).expect("unable to remove to do list file");
   let file= OpenOptions::new()
    .write(true)
    .create(true)
    .append(true)
    .open(TO_DO_FILE)
    .unwrap();

    let mut wtr = csv::Writer::from_writer(file);
    wtr.flush().expect("Failed to flush");

    wtr.write_record(&["task", "date"]).expect("could not write to to do list");
    for task in filtered_to_do_list {
        let date: NaiveDate = task.task_date;
       let date_string =  date_to_string(date);
        wtr.write_record(&[task.task_title, date_string]).expect("could not remove task");

    }
    wtr.flush().expect("Failed to flush");
}

pub fn list_all() {

    let file = File::open(TO_DO_FILE).expect("could not open file path");
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result.expect("could not open dns record");
        let task_title = &record[0];
        let task_date = &record[1];
        println!("\n {} -- {} \n",task_title,task_date);

    }


}
