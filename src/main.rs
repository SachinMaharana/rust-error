use std::collections::HashMap;
use chrono::NaiveDate;

fn main() {
    match get_current_year() {
        Ok(date) => println!("We have time travelled! {}", date),
        Err(e) => eprintln!("Oh Noes, we don't know {}", e)
    }
  
}

fn get_current_year() -> Result<String, reqwest::Error> {
    let url = "https://postman-echo.com/time/object";
    let res = reqwest::blocking::get(url)?.json::<HashMap<String, i32>>()?;

    // let date = res["years"].to_string();

    let formatted_date =  format!("{}-{}-{}", res["years"], res["months"] + 1, res["date"]);
    let parsed_date = NaiveDate::parse_from_str(formatted_date.as_str(), "%Y-%m-%d")?;
    let date = parsed_date.format("%Y %B %d").to_string();

    Ok(date)
}

