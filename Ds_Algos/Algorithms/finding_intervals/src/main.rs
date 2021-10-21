use std::cmp;

fn find_intervals(calendar1: Vec<Vec<String>>, calendar2: Vec<Vec<String>>, daily_bounds1: Vec<String>, daily_bounds2: Vec<String>, meeting_duration: u32) -> Vec<Vec<String>> {
    let updated1: Vec<Vec<String>> = update_calendar(calendar1,daily_bounds1);
    let updated2: Vec<Vec<String>> = update_calendar(calendar2, daily_bounds2);
    let merged: Vec<Vec<String>> = merge_calendar(updated1, updated2);
    let falttened: Vec<Vec<String>> = flatten_time(merged);
    available_intervals(falttened, meeting_duration)
}

fn update_calendar(calendar: Vec<Vec<String>>, daily_bounds: Vec<String>) -> Vec<Vec<String>> {
    let mut temp = calendar.clone();
    temp.insert(0, vec!["00:00".to_string(),daily_bounds[0].clone()]);
    temp.push(vec![daily_bounds[1].clone(),"23:59".to_string()]);
    temp
}

fn merge_calendar(calendar1: Vec<Vec<String>>, calendar2: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut merged: Vec<Vec<String>> = Vec::new();

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < calendar1.len() && j < calendar2.len() {
        let a: u32 = time_to_minutes(calendar1[i][0].clone());
        let b: u32 = time_to_minutes(calendar2[j][0].clone());
        if a < b {
            merged.push(calendar1[i].clone());
            i += 1;
        } else {
            merged.push(calendar2[j].clone());
            j += 1
        }
    }

    while i < calendar1.len() {
        merged.push(calendar1[i].clone());
        i += 1;
    }
    while j < calendar2.len() {
        merged.push(calendar2[j].clone());
        j += 1;
    }
    merged
}

fn flatten_time(merged: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut flattened: Vec<Vec<String>> = Vec::new();
    flattened.push(merged[0].clone());
    for i in 1..merged.len() {
        let current: Vec<String> = merged[i].clone();
        let j: usize = flattened.len();
        let prev: Vec<String> = flattened[j-1].clone();
        let a: u32 = time_to_minutes(current[0].clone());
        let b: u32 = time_to_minutes(prev[1].clone());
        if a <= b {
            let c: u32 = time_to_minutes(current[1].clone());
            let temp = vec![prev[0].clone(),minutes_to_time(cmp::max(b,c))];
            let i: usize = flattened.len();
            flattened[ i - 1] = temp;
        } else {
            flattened.push(current.clone());
        }
    }
    flattened
}

fn available_intervals(flattened: Vec<Vec<String>>, meeting_duration: u32) -> Vec<Vec<String>> {
    let mut intervals: Vec<Vec<String>> = Vec::new();
    for i in 1..flattened.len() {
        let current: Vec<String> = flattened[i].clone();
        let prev: Vec<String> = flattened[i-1].clone();
        let a: u32 = time_to_minutes(current[0].clone());
        let b: u32 = time_to_minutes(prev[1].clone());
        if (a - b) >= meeting_duration {
            intervals.push(vec![prev[1].clone(),current[0].clone()]);
        }
    }
    intervals
}

fn time_to_minutes(time: String) -> u32 {
    let tmp: Vec<&str> = time.split(":").collect();
    let hours: u32 = tmp[0].parse::<u32>().unwrap() * 60;
    let minutes: u32 = tmp[1].parse::<u32>().unwrap() % 60;
    hours + minutes
}

fn minutes_to_time(minutes: u32) -> String {
    let hours: u32 = (minutes / 60) as u32;
    let minutes: u32 = minutes % 60;
    let hour_str: String = { if hours <= 0 { format!("{}{}",0,0) }else{ hours.to_string()} };
    let minutes_str: String = { if minutes <= 0 { format!("{}{}",0,minutes.to_string()) }else{ minutes.to_string()} };
    format!("{}:{}",hour_str,minutes_str)
}


fn main() {
   let calendar1:Vec<Vec<String>> = vec![
    vec!["9:00".to_string(), "10:30".to_string()],
    vec!["12:00".to_string(), "13:00".to_string()],
   vec!["16:00".to_string(), "18:00".to_string()]
  ];
  let daily_bounds1: Vec<String> = vec!["9:00".to_string(), "20:00".to_string()];
  let calendar2: Vec<Vec<String>> = vec![
    vec!["10:00".to_string(), "11:30".to_string()],
    vec!["12:30".to_string(), "14:30".to_string()],
    vec!["14:30".to_string(), "15:00".to_string()],
    vec!["16:00".to_string(), "17:00".to_string()]
  ];
  let daily_bounds2: Vec<String> = vec!["10:00".to_string(), "18:30".to_string()];
  let meeting_duration: u32 = 30;


  let result: Vec<Vec<String>> = find_intervals(calendar1, calendar2, daily_bounds1, daily_bounds2, meeting_duration);

  println!("Result: {:?}", result);
}

