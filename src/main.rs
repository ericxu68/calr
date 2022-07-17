#![allow(dead_code)]
#![allow(unused_variables)]
use chrono::prelude::*;

#[derive(Debug)]
struct Calendar {
    curr_dt: DateTime<Local>,
    num_days: u32,
}

impl Calendar {
    fn new() -> Self {
        Calendar {
            curr_dt: Local::now(),
            num_days: 1,
        }
    }

    /// set number of days from month
    fn set_num_days(&mut self) {
        self.num_days = get_num_days(self.curr_dt.month(), self.curr_dt.year());
    }

    /// show the current calendar of the month
    fn print(&self) {
        // print header
        //println!("{}", self.curr_dt.format("%a %b %e %T %Y").to_string());
        println!("\t{}", self.curr_dt.format("%B %Y").to_string());

        let mut day = Weekday::Sun;
        print!("{} ", day);
        for _i in 1..=6 {
            day = day.succ();
            print!("{} ", day);
        }
        println!("");

        // print days of the week
        let (year, month, day) = get_last_dt(self.curr_dt.month(), self.curr_dt.year());
        //let dt = NaiveDate::from_ymd(year, month, day);

        for day in 1..=self.num_days {
            print!("{:3} ", day);
        }

        println!("");
    }
}

fn get_num_days(month: u32, year: i32) -> u32 {
    let c_year = if month == 12 { year + 1 } else { year };

    let c_mount = if month == 12 { 1 } else { month + 1 };

    let date = NaiveDate::from_ymd(c_year, c_mount, 1);
    date.signed_duration_since(NaiveDate::from_ymd(year, month, 1))
        .num_days() as u32
}

fn get_last_dt(month: u32, year: i32) -> (i32, u32, u32) {
    let last_month = if month == 1 { 12 } else { month - 1 };

    let last_year = if month == 1 { year - 1 } else { year };

    (last_year, last_month, get_num_days(last_month, last_year))
}

fn main() {
    let mut calendar = Calendar::new();
    calendar.set_num_days();
    calendar.print();
}
