use std::collections::HashMap;

fn main() {
    match get_current_year() {
        Ok(date) => println!("We have time travelled! {}", date),
        Err(e) => eprintln!("Oh Noes, we don't know {}", e)
    }
  
}

fn get_current_year() -> Result<String, reqwest::Error> {
    let url = "https://postman-echo.com/time/object";
    let result = reqwest::blocking::get(url);

    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err)
    };


    let body = response.json::<HashMap<String, i32>>();

    let json = match body {
        Ok(json) => json,
        Err(err) => return Err(err),
    };

    let date = json["years"].to_string();

    Ok(date)
}

