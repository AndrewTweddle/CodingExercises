use std::time::Instant;

const MONTH_LENGTHS: [u8; 12] = [
    31, // Jan
    28, // Feb
    31, // Mar
    30, // Apr
    31, // May
    30, // Jun
    31, // Jul
    31, // Aug
    30, // Sep
    31, // Oct
    30, // Nov
    31, // Dec
];

const FEB_INDEX: usize = 1;
const DAYS_IN_WEEK: u8 = 7;
const SUNDAY_INDEX: u8 = 0;
const MONDAY_INDEX: u8 = 1;

fn main() {
    let start = Instant::now();

    let mut start_day = MONDAY_INDEX;
    let mut months_starting_on_sunday = 0;
    for y in 1900..=2000 {
        for (m, month_len) in MONTH_LENGTHS.iter().enumerate() {
            if start_day == SUNDAY_INDEX && y != 1900 {
                months_starting_on_sunday += 1;
            }
            let is_leap_month = (m == FEB_INDEX) && (y % 4 == 0) && (y % 100 != 0 || y % 400 == 0);
            let days_in_month = month_len + if is_leap_month { 1 } else { 0 };

            // Calculate start date of next month
            start_day += days_in_month;
            start_day %= DAYS_IN_WEEK;
        }
    }
    println!("Months starting on a Sunday: {}", months_starting_on_sunday);

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
