/* Problem 19: Counting Sundays
 *
 * You are given the following information, but you may prefer to do some research for yourself.
 *
 *   1 Jan 1900 was a Monday.
 *
 *   Thirty days has September,
 *   April, June and November.
 *   All the rest have thirty-one,
 *   Saving February alone,
 *   Which has twenty-eight, rain or shine.
 *   And on leap years, twenty-nine.
 *
 *   A leap year occurs on any year evenly divisible by 4, but not on a century unless it is
 *   divisible by 400.
 *
 * How many Sundays fell on the first of the month during the twentieth century
 * (1 Jan 1901 to 31 Dec 2000)? */

#[macro_use]
extern crate enum_primitive;
extern crate num;
use num::{FromPrimitive, Integer};

fn main() {
    let initial_day = Day {
        number: 1,
        month: Month::January,
        weekday: Weekday::Monday,
        year: Year { number: 1900 }
    };

    let mut current: Day = initial_day;
    while !(current.month == Month::December && current.number == 31) {
        current = current.next();
    }

    let mut count = 0usize;

    while current.year.number < 2000 || current.month < Month::December || current.number < 31 {
        if current.number == 1 && current.weekday == Weekday::Sunday {
            count += 1;
        }

        current = current.next();
    }

    println!("{}", count);
}

#[derive(Debug)]
struct Day {
    number: usize,
    weekday: Weekday,
    month: Month,
    year: Year,
}

impl Day {
    fn next(&self) -> Day {
        let next_weekday = self.weekday.next();

        if self.number < self.month.day_count(self.year) {
            Day {
                number:  self.number + 1,
                month:   self.month,
                year:    self.year,
                weekday: next_weekday
            }
        } else {
            let year = match self.month {
                Month::December => self.year.next(),
                _               => self.year
            };

            Day {
                number:  1,
                month:   self.month.next(),
                year:    year,
                weekday: next_weekday
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Year {
    number: usize,
}

impl Year {
    fn is_leap(&self) -> bool {
        4usize.is_multiple_of(&self.number) && !400usize.is_multiple_of(&self.number)
    }

    fn next(&self) -> Year {
        Year { number: self.number + 1 }
    }
}

enum_from_primitive! {
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
}

impl Month {
    fn day_count(self, year: Year) -> usize {
        match self {
            Month::September | Month::April | Month::June | Month::November => 30,
            Month::February => if year.is_leap() { 29 } else { 28 },
            _               => 31,
        }
    }

    fn next(self) -> Month {
        let next_month = (self as isize) + 1;
        let converted = FromPrimitive::from_isize(next_month);

        match converted {
            Some(month) => month,
            None        => Month::January,
        }
    }
}

enum_from_primitive! {
#[derive(PartialEq, Debug, Copy, Clone)]
enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
}

impl Weekday {
    fn next(self) -> Weekday {
        let next_day = (self as isize) + 1;
        let converted = FromPrimitive::from_isize(next_day);

        match converted {
            Some(day) => day,
            None      => Weekday::Sunday,
        }
    }
}
