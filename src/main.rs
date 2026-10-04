mod load_timezones;

use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use jiff::civil::DateTime;
use jiff::tz::TimeZone;
use jiff::Timestamp;
use reqwest::blocking::Client;
use serde::Deserialize;

const API_URL: &str = "https://timeapi.io/api/v1/timezone/zone";

#[derive(Deserialize)]
struct TimeapiResponse {
    has_dst: bool,
    #[serde(flatten)]
    dst_data: Option<DstData>,
    local_time: DateTime,
}

#[derive(Deserialize)]
struct DstData {
    dst_offset_seconds: i32,
    dst_active: bool,
    dst_from: Timestamp,
    dst_until: Timestamp,
}

fn main() -> Result<()> {
    let timezones = load_timezones::load_from_json()?;
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let now = Timestamp::now();
    let system_tz = TimeZone::system();

    let mut earliest_next_change: Option<Timestamp> = None;
    let mut message: String;
    for tz in timezones {
        match fetch_timeapi_response(&client, &tz.name) {
            Ok(tz_data) => match tz_data.dst_data {
                Some(dst_data) => {
                    match dst_data.dst_active {
                        true => message = String::from("DST"),
                        false => message = String::from("OFF"),
                    }

                    message.push_str(&format!(", LT: {}", format_time(tz_data.local_time)));

                    let next_change = [dst_data.dst_from, dst_data.dst_until]
                        .into_iter()
                        .filter(|t| *t > now)
                        .min()
                        .expect("Error computing next change");
                    if earliest_next_change.is_none_or(|change| next_change < change) {
                        earliest_next_change = Some(next_change);
                    }
                    message.push_str(&format!(
                        ". Change: {}.",
                        next_change.to_zoned(system_tz.clone()).strftime("%c %Z")
                    ));

                    if dst_data.dst_offset_seconds.abs() != 60 * 60 {
                        message.push_str(&format!(
                            " Non-standard DST offset: {}",
                            dst_data.dst_offset_seconds
                        ));
                    }
                }
                None => {
                    match tz_data.has_dst {
                        false => message = String::from("---"),
                        true => message = String::from("Missing DST data"),
                    }
                    message.push_str(&format!(", LT: {}", format_time(tz_data.local_time)));
                }
            },
            Err(err) => {
                message = format!("Failed to load data: {:?}", err);
            }
        }
        println!("{} ({}): {message}", tz.code, tz.name);
        sleep(Duration::from_millis(500));
    }
    println!();
    match earliest_next_change {
        Some(timestamp) => {
            println!(
                "Earliest next change: {}",
                timestamp.to_zoned(system_tz).strftime("%c %Z")
            );
        }
        None => {
            println!("No changes planned.")
        }
    };
    Ok(())
}

fn fetch_timeapi_response(client: &Client, tz_name: &str) -> Result<TimeapiResponse> {
    let data: TimeapiResponse = client
        .get(API_URL)
        .query(&[("timeZone", tz_name)])
        .send()?
        .error_for_status()?
        .json()?;
    Ok(data)
}

fn format_time(datetime: DateTime) -> String {
    datetime.strftime("D%d %H:%M").to_string()
}
