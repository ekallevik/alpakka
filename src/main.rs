use std::collections::{HashMap, HashSet};
use std::ops::Add;
use chrono::{Days, Utc, Weekday};
use inquire::{DateSelect, MultiSelect, Text};
use anyhow::Result;

fn main() -> Result<()> {

    let now = Utc::now();
    let trip_name = Text::new("Enter trip name...").prompt()?;

    let start_date = DateSelect::new("When do you want to travel?")
        .with_min_date(now.date_naive())
        .with_starting_date(now.date_naive())
        .with_week_start(Weekday::Mon)
        .prompt()?;

    let end_date = DateSelect::new("When are you returning?")
        .with_min_date(start_date)
        .with_starting_date(start_date.add(Days::new(1)))
        .with_week_start(Weekday::Mon)
        .prompt()?;

    let mut activities = HashMap::new();
    activities.insert("business", vec!["computer", "charger", "headset", "phone charger"]);
    activities.insert("climbing", vec!["rope", "climbing shoes", "harness", "chalk", "sunscreen", "climbing pant", "wool tee"]);
    activities.insert("running", vec!["running shoes", "shoes", "sunscreen", "running shorts", "training tee"]);
    activities.insert("hiking", vec!["hiking shoes", "sun screen", "sun glasses", "hiking apparel"]);

    let keys = activities.keys().clone().collect();
    let selected_activities = MultiSelect::new("Select activities", keys).prompt()?;

    let items: Vec<&str> = selected_activities
        .iter()
        .flat_map(|&key| activities.get(key)?.to_owned())
        .collect();

    let unique_items: HashSet<_> = HashSet::from_iter(items.iter().copied());

    println!("Creating trip: {trip_name}, starting on {start_date} and lasting until {end_date}");
    println!("Selected: {:?}", selected_activities);

    for i in unique_items {
        println!("- {i}")
    };

    Ok(())
}

