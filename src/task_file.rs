extern crate csv;
use std::fs;
use std::fs::File;
use crate::TO_DO_FILE;
use std::fs::OpenOptions;
use crate::Task;
use chrono::NaiveDate;
use std;
use crate::date_functions::{string_to_date,date_to_string,difference_between_dates};
use colored::Colorize;

fn parse_ics_string(content: &str) -> (Vec<String>,Vec<String>) {
    let ics_lines: Vec<&str> = content.split("\n").collect();
    let ics_summaries: Vec<String> = ics_lines
        .clone()
        .into_iter()
        .filter(|string| string.contains("SUMMARY"))
        .map(|string| string.replace("SUMMARY:",""))
        .collect();


    let ics_dates: Vec<String> = ics_lines.into_iter()
        .filter(|string| string.contains("DTSTART"))
        .map(|string| string.replace("DTSTART:",""))
        .map(|string| string.split("T").next().expect("Could not split on T").to_string())
        .map(|string| format!("{}-{}-{}",&string[0..4],&string[4..6],&string[6..8]))
        .collect();
    (ics_summaries,ics_dates)
}

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

pub fn import_ics(file_path: &str) {

    /* Load in file to string */
    let file_ics_content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let tasks = parse_ics_string(&file_ics_content);



    /* Opening file once before for loop to be more efficient than calling add task every time */
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(TO_DO_FILE)
        .unwrap();
    let mut wtr = csv::Writer::from_writer(file);

    for i in 0..tasks.0.len() {
        let task_title = &tasks.0[i];
        let task_date = &tasks.1[i];
        wtr.write_record(&[task_title, task_date]).expect("could not write to to do list");
        wtr.flush().expect("Failed to flush");
    }
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
    let length_before_filter = to_do_list.len();
    let filtered_to_do_list: Vec<Task> = to_do_list.into_iter().filter(|task| task.task_title != task_title).collect();
    if filtered_to_do_list.len() == length_before_filter {
        println!("Task does not exist in record");
        std::process::exit(0);
    }


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
    println!("\n listing in order added, not order of importance \n");

    let file = File::open(TO_DO_FILE).expect("could not open file path");
    let mut rdr = csv::Reader::from_reader(file);
    let mut to_do_list: Vec<Task> = Vec::new();

    for result in rdr.records() {
        let record = result.expect("could not open dns record");
        let task_title = &record[0];
        let task_date = &record[1];
        let dt = string_to_date(task_date);
        let new_task = Task { task_title: task_title.to_owned(), task_date: dt };
        to_do_list.push(new_task);
    }

    to_do_list.sort_by(|task_a, task_b| task_b.task_date.cmp(&task_a.task_date));
    let today_nav = chrono::offset::Local::now().date_naive();

    for task in to_do_list {
        let diff = difference_between_dates(task.task_date,today_nav);

        if diff<0 {
            println!("------ \n task: {} \n date: {} \n------ ",task.task_title, date_to_string(task.task_date).red());
        }
        else if diff<2 {
            println!("------ \n task: {} \n date: {} \n------ ",task.task_title, date_to_string(task.task_date).yellow());
        }
        else if diff <=7 {
            println!("------ \n task: {} \n date: {} \n------ ",task.task_title, date_to_string(task.task_date).green());
        }
        else {
            println!("------ \n task: {} \n date: {} \n------ ",task.task_title, date_to_string(task.task_date));
        }
        }
}
