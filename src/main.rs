use chrono::prelude::*;
// use chrono::Duration;
use clap::Parser;
use clap::Subcommand;
use duration_str::parse_chrono;
// use std::io::{self, Read};

#[derive(Parser, Debug)]
#[clap(author="Senh Mo Chuang senhmo.chuang@amd.com", version="0.0.1", about="Time Differ CLI", long_about = None)]
struct Timewarp {
    /// Reference Time: This time will add with offset
    #[clap(short, long)]
    ref_time: String,

    /// Offset: A descriptive description for offset to be added to Reference Time
    /// with the following units:
    /// <nanoseconds microseconds milliseconds seconds minutes hours days weeks months years>
    /// ie: "15 days 20 seconds 100 milliseconds"
    #[clap(short, long)]
    offset: String,

    /// Comparative Time or Filename to list of Dates with the proper format: This time will be compared to Reference Time + Offset
    #[clap(short, long)]
    comp_timeorfile: String,

    /// Format Time: "%a %b %e %T %Y" is default.
    /// See reference here: https://strftime.org/
    #[clap(short, long)]
    format: Option<String>,

    #[clap(subcommand)]
    command: Comparator,
}

#[derive(Subcommand, Debug)]
enum Comparator {
    ///Comparing Sample Time > Reference + Offset
    Gt,
    ///Comparing Sample Time < Reference + Offset
    Lt,
    ///Comparing Sample Time = Reference + Offset
    Eq,
    ///Comparing Sample Time >= Reference + Offset
    Ge,
    ///Comparing Sample Time <= Reference + Offset
    Le,
}

fn main() {
    let timewarp = Timewarp::parse();
    let mut date_format = "%a %b %e %T %Y".to_owned();
    if timewarp.format.is_some() {
        date_format = timewarp.format.unwrap();
    }
    // let time1 = Local.datetime_from_str(&args.d1, "%a %b %e %T %Y").unwrap();
    let reference_time = Local
        .datetime_from_str(&timewarp.ref_time, &date_format)
        .unwrap();
    let sample_time = Local
        .datetime_from_str(&timewarp.comp_timeorfile, &date_format)
        .unwrap();
    match timewarp.command {
        Comparator::Gt => println!(
            "{}",
            sample_time > reference_time + parse_chrono(timewarp.offset).unwrap()
        ),
        Comparator::Lt => println!(
            "{}",
            sample_time < reference_time + parse_chrono(timewarp.offset).unwrap()
        ),
        Comparator::Eq => println!(
            "{}",
            sample_time == reference_time + parse_chrono(timewarp.offset).unwrap()
        ),
        Comparator::Le => println!(
            "{}",
            sample_time <= reference_time + parse_chrono(timewarp.offset).unwrap()
        ),
        Comparator::Ge => println!(
            "{}",
            sample_time >= reference_time + parse_chrono(timewarp.offset).unwrap()
        ),
    }

    // std::path::Path::new(fp).exists()
    // let mut buffer = String::new();
    // let standard_input = io::stdin();
    // standard_input.lock().read_to_string(&mut buffer).unwrap();
    // for each in buffer.trim().split("\n") {
    //     let converted_time = Local.datetime_from_str(each, "%a %b %e %T %Y").unwrap();
    //     println!("{}", Utc::now().signed_duration_since(converted_time));
    //     println!(
    //         "{}",
    //         Utc::now().signed_duration_since(converted_time) + parse_chrono("3d").unwrap()
    //     );
    //     println!("{}", Duration::days(3));
    // }
    // println!("{}", Utc::now());
    // println!("{}", Local::now());
    // println!(
    //     "{}",
    //     FixedOffset::east(9 * 3600).ymd(2014, 11, 28) // .and_hms_nano(21, 45, 59, 324310806)
    // );
    // println!("{}:{}", local.hour(), local.minute());
    // println!("{}", local.time().format("%H:%M:%S"));
}
