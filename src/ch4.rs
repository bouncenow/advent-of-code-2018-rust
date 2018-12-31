use std::collections::HashMap;

use chrono::prelude::*;

use crate::common::read_lines_from_file;

pub fn ch4() {
    let events = read_events_from_file_sorted("ch4.txt");
    for e in &events {
        println!("{:?}", e);
    }
    let sleeps_grouped = build_sleeps_grouped_by_guard(&events).unwrap_or_else(|e| panic!("Error during processing events: {}", e));
    
    strategy1(&sleeps_grouped);
    strategy2(&sleeps_grouped);
}

fn strategy1(sleeps_grouped: &HashMap<usize, Vec<Sleep>>) {
    let (guard_id, sleeps) = find_guard_with_most_slept_amount(sleeps_grouped);
    
    let (most_slept_minute_for_guard_with_most_sleep, _) = find_most_slept_minute(sleeps);
    println!("Strategy 1: {} * {} = {}", most_slept_minute_for_guard_with_most_sleep, guard_id, most_slept_minute_for_guard_with_most_sleep * guard_id);
}

fn find_guard_with_most_slept_amount<'a>(sleeps_grouped: &'a HashMap<usize, Vec<Sleep>>) -> (&'a usize, &'a Vec<Sleep>) {
    sleeps_grouped.iter()
        .max_by_key(|&(_, sleeps)| sleeps.iter().map(|s| s.len()).sum::<usize>())
        .unwrap_or_else(|| panic!("Not single guard was in event log"))
}

fn strategy2(sleeps_grouped: &HashMap<usize, Vec<Sleep>>) {
    let (guard_id, most_slept_minute) = fing_guard_with_most_slept_minute(sleeps_grouped);
    println!("Strategy 2: {} * {} = {}", most_slept_minute, guard_id, guard_id * most_slept_minute);
}

fn fing_guard_with_most_slept_minute<'a>(sleeps_grouped: &'a HashMap<usize, Vec<Sleep>>) -> (&'a usize, usize) {
    sleeps_grouped.iter()
        .map(&|(guard_id, sleeps)| (guard_id, find_most_slept_minute(sleeps)))
        .max_by_key(|&(_, (_, slept))| slept)
        .map(|(guard_id, (most_slept_minute, _))| (guard_id, most_slept_minute))
        .unwrap_or_else(|| panic!("Not single guard was in event log"))
}

const MINUTES: usize = 60;

fn find_most_slept_minute(sleeps: &Vec<Sleep>) -> (usize, usize) {
    let mut buf: Vec<usize> = vec![0; MINUTES];
    for s in sleeps {
        for i in s.from..(s.to + 1) {
            buf[i] += 1;
        }
    }
    buf.into_iter().enumerate().max_by_key(|(_, slept)| *slept).unwrap()
}

fn build_sleeps_grouped_by_guard(events: &Vec<Event>) -> Result<HashMap<usize, Vec<Sleep>>, String> {
    let mut sleeps_grouped = HashMap::new();
    let mut sleep_state = PartialSleepState::NoGuard;
    for e in events {
        match e.e_type {
            EventType::ShiftBegin(guard_id) => match sleep_state {
                PartialSleepState::NoGuard | PartialSleepState::NotSleeping(_) => sleep_state = PartialSleepState::NotSleeping(guard_id),
                _ => return unexpected_state_for_event(e, &sleep_state)
            },
            EventType::WakeUp => match sleep_state {
                PartialSleepState::Sleeping(guard_id, from) => {
                    sleep_state = PartialSleepState::NotSleeping(guard_id);
                    let to = if e.date.minute() > 0 {
                        (e.date.minute() - 1) as usize
                    } else {
                        return Err("Woke up at 0 minute".to_string())
                    };
                    let sleep = Sleep { from, to };
                    let sleeps = sleeps_grouped.entry(guard_id).or_insert(Vec::new());
                    (*sleeps).push(sleep);
                }
                _ => return unexpected_state_for_event(e, &sleep_state)
            }
            EventType::FallAsleep => match sleep_state {
                PartialSleepState::NotSleeping(guard_id) => sleep_state = PartialSleepState::Sleeping(guard_id, e.date.minute() as usize),
                _ => return unexpected_state_for_event(e, &sleep_state)
            }
        }
    }
    Ok(sleeps_grouped)
}

fn unexpected_state_for_event<T>(e: &Event, state: &PartialSleepState) -> Result<T, String> {
    Err(format!("Unexpected state for event {:?}: {:?}", e, state))
}

#[derive(Debug)]
enum PartialSleepState {
    NoGuard,
    NotSleeping(usize),
    Sleeping(usize, usize)
}

struct Sleep {
    from: usize,
    to: usize
}

impl Sleep {
    fn len(&self) -> usize {
        self.to - self.from + 1
    }
}

#[derive(Debug)]
enum EventType {
    WakeUp,
    FallAsleep,
    ShiftBegin(usize)
}

#[derive(Debug)]
struct Event {
    e_type: EventType,
    date: DateTime<Utc>
}

fn read_events_from_file_sorted(file_name: &str) -> Vec<Event> {
    let mut events: Vec<Event> = read_lines_from_file(file_name).iter()
        .map(|l| Event::parse(l.trim()).unwrap_or_else(|e| panic!("Error during event parsing: {}", e)))
        .collect();

    events.sort_unstable_by_key(|e| e.date);

    events
}

impl Event {
    fn parse(str: &str) -> Result<Event, String>  {
        let (date_part, e_type_part) = scan_fmt!(str, "[{/[0-9-: ]+/}] {/[A-Za-z#0-9 ]+/}", String, String);

        let date = if let Some(date_part) = date_part {
            match Utc.datetime_from_str(&date_part, "%Y-%m-%d %H:%M") {
                Ok(d) => d,
                Err(e) => return Err(format!("Couldn't parse date, error: {} in event {}", e, str))
            }
        } else {
            return Err(format!("Couldn't find date part in event {}", str));
        };

        let e_type = if let Some(e_type_part) = e_type_part {
            match e_type_part.as_ref() {
                "wakes up" => EventType::WakeUp,
                "falls asleep" => EventType::FallAsleep,
                _ => {
                    let guard_id = scan_fmt!(&e_type_part, "Guard #{d} begins shift", usize);
                    if let Some(guard_id) = guard_id {
                        EventType::ShiftBegin(guard_id)
                    } else {
                        return Err(format!("Unable to parse e_type: {}", &e_type_part));
                    }
                }
            }
        } else {
            return Err(format!("Couldn't find e_type part in event {}", str));
        };

        Ok(Event { e_type, date })
    }
}
