mod load_timezones;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TimeapiResponse {
    has_dst: bool,
    dst_offset_seconds: i32,
    dst_active: bool,
    dst_from: String,
    dst_until: String,
    local_time: String,
}

fn fetch_timeapi_response(tz_name: &str) -> Result<TimeapiResponse> {
    let tz_name = urlencoding::encode(tz_name);
    let url = format!("https://timeapi.io/api/v1/timezone/zone?timeZone={}", tz_name);
    let res = reqwest::blocking::get(url)?;
    let data: TimeapiResponse = res.json()?;
    Ok(data)
}

fn main() -> Result<()> {
    let timezones = load_timezones::load_from_json(None)?;
    for tz in timezones {
        let tz_data = fetch_timeapi_response(&tz.name)?;
        let dst_state = match tz_data.dst_active {
            true => "DST",
            false => "---",
        };
        println!("{}: {}, {}", tz.code, dst_state, tz_data.local_time);
    }
    Ok(())
}
