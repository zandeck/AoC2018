use nom::*;
use chrono::prelude::*;
use std::str::FromStr;
use super::common;
use std::collections::{HashSet, HashMap};
use time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    id: u32
}

impl Guard {
    fn new(id: u32) -> Guard {
        Guard {id}
    }
}

#[derive(Debug, Clone, Copy)]
enum EventType {
    FallAsleep,
    WakeUp,
    StartShift
}

#[derive(Debug, Clone, Copy)]
struct Event {
    guard: Option<Guard>,
    dt: DateTime<Utc>,
    event_type: EventType
}

impl Event {
     fn new(guard: Option<Guard>, dt: DateTime<Utc>, event_type: EventType) -> Event {
         Event {guard, dt, event_type}
     }

     fn set_guard(&mut self, guard: Guard) {
         self.guard = Some(guard);
     }
}

#[derive(Debug, Clone)]
struct Shift {
    guard: Guard,
    events: Vec<Event>
}

impl Shift {
    fn new(guard: Guard, events: Vec<Event>) -> Shift {
        Shift {guard, events}
    }

    fn time_asleep(&self) -> i64{
        let mut total = Duration::minutes(0);
        let mut previous_time = self.events[0].dt;

        for events in &self.events {
            match &events.event_type {
                EventType::WakeUp => total = total + (events.dt - previous_time),
                EventType::FallAsleep => previous_time = events.dt,
                _ => ()
            }
        }
        total.num_minutes()
    }

    fn count_minutes(t1: DateTime<Utc>, t2: DateTime<Utc>, minutes: &mut HashMap<i64, i64>) {
        for m in t1.minute()..t2.minute() {
            let v = minutes.entry(m as i64).or_insert(0);
            *v += 1;
        }
    }

    fn minutes_spent_sleeping(&self) -> HashMap<i64, i64> {
        let mut minutes: HashMap<i64, i64> = HashMap::new();
        let mut previous_time = self.events[0].dt;
        for event in &self.events {
            match &event.event_type {
                EventType::WakeUp => Shift::count_minutes(previous_time, event.dt, &mut minutes),
                EventType::FallAsleep => previous_time = event.dt,
                _ => ()
            }
        }

        minutes
    }
}

#[derive(Debug)]
struct Shifts {
    shifts: Vec<Shift>
}

impl Shifts {
    fn new(events: &Vec<Event>) -> Shifts {
        let mut shifts: Vec<Shift> = Vec::new();
        let mut current_guard: Guard = Guard::new(0);
        let mut current_events: Vec<Event> = Vec::new();

        for event in events {
            match &event.guard {
                Some(g) => {
                    if !current_events.is_empty() {
                        let shift = Shift::new(current_guard, current_events);
                        shifts.push(shift);
                    } 
                    current_guard = *g;
                    current_events = vec! [ *event ];
                },
                None => current_events.push( *event ),
            }

        }
        let shift = Shift::new(current_guard, current_events);
        shifts.push(shift);

        Shifts {shifts}

    }

    fn guards_on_shift(&self) -> HashSet<Guard> {
        let mut guards: HashSet<Guard> = HashSet::new();
        for shift in &self.shifts {
            guards.insert(shift.guard);
        }

        guards
    }

    fn time_guards_asleep(&self) -> HashMap<Guard, i64>{
        self.minutes_spent_asleep().into_iter().map(|(k, v)| (k, v.values().sum()) ).collect()
    }

    fn most_asleep_guard(&self) -> Guard {
        let times = &self.time_guards_asleep();
        *times.into_iter().max_by(|e1, e2| e1.1.cmp(&e2.1)).unwrap().0
    }

    fn merge_minutes(h1: &HashMap<i64, i64>, h2: &HashMap<i64, i64>) -> HashMap<i64, i64> {
        let mut h: HashMap<i64, i64> = h1.clone();
        
        for (k, v) in h2 {
            let vv = h.entry(*k).or_insert(0);
            *vv += v;
        }
        h
    }

    fn minutes_spent_asleep(&self) -> HashMap<Guard, HashMap<i64, i64>> {
        let mut res: HashMap<Guard, HashMap<i64, i64>> = HashMap::new();
        for shift in &self.shifts {
            let minutes = shift.minutes_spent_sleeping();
            let d = res.entry(shift.guard).or_insert(HashMap::new());
            *d = Shifts::merge_minutes(d, &minutes);
        }
        res
    }

    fn most_minute_aslept(&self) -> HashMap<Guard, i64> {
        let minutes = self.minutes_spent_asleep();
        minutes.into_iter().map(|(k, v)| (k, v.into_iter().max_by_key(|x| x.1 ).unwrap_or((0,0)).0) ).collect()
    }

    fn most_minute_aslept2(&self) -> HashMap<Guard, (i64, i64)> {
        let minutes = self.minutes_spent_asleep();
        minutes.into_iter().map(|(k, v)| (k, v.into_iter().max_by_key(|x| x.1 ).unwrap_or((0,0))) ).collect()
    }


    fn get_key(&self) -> i64 {
        let most_freq_minutes: HashMap<Guard, i64> = self.most_minute_aslept();
        let most_asleep: Guard = self.most_asleep_guard();
        println!("{:?}", most_freq_minutes);
        println!("{:?}", most_asleep);
        *most_freq_minutes.get(&most_asleep).unwrap() * (most_asleep.id as i64)
    }

    fn get_key2(&self) -> i64 {
        let (g, (minute, freq)): (Guard, (i64, i64)) = self.most_minute_aslept2().into_iter().max_by_key(|(g, m)| m.1 ).unwrap();
        (g.id as i64) * minute
    }
}

fn u8_to_u32(l: nom::types::CompleteStr) -> i32 {
    i32::from_str(&l).unwrap()
}

named!(parse_date< nom::types::CompleteStr, DateTime<Utc> >, 
    do_parse!(
        ws!(tag!("[")) >>
        Y: ws!(digit) >>
        ws!(tag!("-")) >>
        M: ws!(digit) >>
        ws!(tag!("-")) >>
        D: ws!(digit) >>
        h: ws!(digit) >>
        ws!(tag!(":")) >>
        m: ws!(digit) >>
        ws!(tag!("]")) >>
        (Utc.ymd(
            u8_to_u32(Y),
            u8_to_u32(M) as u32,
            u8_to_u32(D) as u32)
            .and_hms( 
                u8_to_u32(h) as u32,
                u8_to_u32(m) as u32,
                0))
    )
);

named!(parse_event_guard_start< nom::types::CompleteStr,Event>,
    do_parse!(
        dt: ws!(parse_date) >>
        ws!(tag!("Guard #")) >>
        id: ws!(digit) >>
        ws!(tag!("begins shift")) >>
        (Event::new( Some(Guard::new(u8_to_u32(id) as u32)), dt, EventType::StartShift))
    )
 );

named!(parse_event_guard_fallasleep< nom::types::CompleteStr,Event>,
    do_parse!(
        dt: ws!(parse_date) >>
        ws!(tag!("falls asleep")) >>
        (Event::new( None, dt, EventType::FallAsleep))
    )
 );

named!(parse_event_guard_wakeup< nom::types::CompleteStr,Event>,
    do_parse!(
        dt: ws!(parse_date) >>
        ws!(tag!("wakes up")) >>
        (Event::new( None, dt, EventType::WakeUp))
    )
 );

named!(parse_event_guard< nom::types::CompleteStr,Event>,
    alt!( ws!(parse_event_guard_start) | ws!(parse_event_guard_fallasleep) | ws!(parse_event_guard_wakeup))
);

named!(parse_events< nom::types::CompleteStr,Vec<Event>>,
    many0!(ws!(parse_event_guard))
);

pub fn part1() {
    let data = common::import_file_to_u8("resources/input4.txt".to_string()).unwrap();
    let test_date = nom::types::CompleteStr(&data);
    let mut events: Vec<Event> = parse_events(test_date).ok().unwrap().1;
    //println!("{:?}", events);

    events.sort_by(|e1, e2| e1.dt.cmp(&e2.dt) );
    let shifts = Shifts::new(&events);
    println!("Guards on shift: {:?}", shifts.guards_on_shift());

    println!("Most asleept guard: {:?}", shifts.most_asleep_guard());
    println!("Time asleep: {:?}", shifts.time_guards_asleep());

    println!("Most minute asleep: {:?}", shifts.most_minute_aslept());

    println!("Answer: {:?}", shifts.get_key());
    
    println!("Answer2: {:?}", shifts.get_key2());

}