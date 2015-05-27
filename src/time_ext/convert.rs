// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::ops::{Ordering};
use core::cmp::{Ord};

use super::{Zone, DateTime, Weekday, Time};

const SECS_PER_MIN:         i64 = 60;
const MINS_PER_HOUR:        i64 = 60;
const HOURS_PER_DAY:        i64 = 24;
const DAYS_PER_WEEK:        i64 = 7;
const DAYS_PER_NORMAL_YEAR: i64 = 365;
const DAYS_PER_LEAP_YEAR:   i64 = 366;
const SECS_PER_HOUR:        i64 = SECS_PER_MIN * MINS_PER_HOUR;
const SECS_PER_DAY:         i64 = SECS_PER_HOUR * HOURS_PER_DAY;
const MONTHS_PER_YEAR:      i64 = 12;

const EPOCH_YEAR: i64 = 1970;
const EPOCH_WEEK_DAY: Weekday = Weekday::Thursday;

static DAYS_PER_MONTH: [[i8; 12]; 2] = [
	[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
	[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
];

static DAYS_PER_YEAR: [i64; 2] = [
    DAYS_PER_NORMAL_YEAR, DAYS_PER_LEAP_YEAR,
];

/// Returns `1` is the year is a leap year, `0` otherwise.
fn is_leap(year: i64) -> usize {
    (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)) as usize
}

/// Returns the total number of leap years that occured in the interval [0, year].
fn leap_years_to(year: i64) -> i64 {
    if year >= 0 {
        year / 4 - year / 100 + year / 400
    } else {
        -leap_years_to(-year - 1) - 1
    }
}

pub fn explode(zone: &Zone, time: i64) -> DateTime {
    let state = if zone.transitions.len() == 0 || time < zone.transitions[0].0 {
        match zone.states.find(|s| !s.1) {
            Some(i) => zone.states[i],
            _ => zone.states[0],
        }
    } else {
        match zone.transitions.find_binary(|t| t.0.cmp(&time)) {
            (Some(n), _) => zone.states[zone.transitions[n].1],
            (_, n) => zone.states[zone.transitions[n - 1].1],
        }
    };

    let mut is_leap_second = false;
    let leap_seconds = match zone.leap_seconds.find_reverse(|l| time >= l.0) {
        Some(i) => {
            is_leap_second = time == zone.leap_seconds[i].0;
            zone.leap_seconds[i].1
        },
        _ => 0,
    };

    let mut year = EPOCH_YEAR;
    let (mut days, mut secs) = time.div_rem(SECS_PER_DAY);

    while days < 0 || days >= DAYS_PER_YEAR[is_leap(year)] {
        let next_year = year + match days / DAYS_PER_LEAP_YEAR {
            0 if days < 0 => -1,
            0 => 1,
            n => n,
        };
        let leap_days = leap_years_to(next_year - 1) - leap_years_to(year - 1);
        days -= (next_year - year) * DAYS_PER_NORMAL_YEAR;
        days -= leap_days;
        year = next_year;
    }

    secs += state.0 - leap_seconds;
    let (div_secs, mut secs) = secs.div_rem(SECS_PER_DAY);
    days += div_secs;
    if secs < 0 {
        days -= 1;
        secs += SECS_PER_DAY;
    }

    while days < 0 {
        year -= 1;
        days += DAYS_PER_YEAR[is_leap(year)];
    }
    while days >= DAYS_PER_YEAR[is_leap(year)] {
        days -= DAYS_PER_YEAR[is_leap(year)];
        year -= 1;
    }

    let weekday = ((((EPOCH_WEEK_DAY as i64) + ((year - EPOCH_YEAR) % DAYS_PER_WEEK) *
                    (DAYS_PER_NORMAL_YEAR % DAYS_PER_WEEK) + leap_years_to(year - 1) -
                    leap_years_to(EPOCH_YEAR - 1) + days) % DAYS_PER_WEEK) + 
                    DAYS_PER_WEEK) % DAYS_PER_WEEK;
    let weekday = match weekday {
        0 => Weekday::Monday,
        1 => Weekday::Tuesday,
        2 => Weekday::Wednesday,
        3 => Weekday::Thursday,
        4 => Weekday::Friday,
        5 => Weekday::Saturday,
        _ => Weekday::Sunday,
    };

    let (hour, secs) = secs.div_rem(SECS_PER_HOUR);
    let (min, mut secs) = secs.div_rem(SECS_PER_MIN);
    secs += is_leap_second as i64;

    let mut day_in_month = days;

    let mut month = 0;
    while day_in_month >= DAYS_PER_MONTH[is_leap(year)][month] as i64 {
        day_in_month -= DAYS_PER_MONTH[is_leap(year)][month] as i64;
        month += 1;
    }
    day_in_month += 1;
    month += 1;

    DateTime {
        offset:      state.0,
        year:        year,
        day_in_year: days as i16 + 1,
        month:       month as i8,
        day:         day_in_month as i8,
        hour:        hour as i8,
        minute:      min as i8,
        second:      secs as i8,
        weekday:     weekday,
        summer_time: state.1,
    }
}

pub fn compact(zone: &Zone, mut date: DateTime) -> (DateTime, Time) {
    normalize(&mut date);

    let transition = find_transition(zone, &date);

    let (state, mut lo, mut hi) = match transition {
        Some(n) => {
            let (time, idx) = zone.transitions[n];
            if n + 1 < zone.transitions.len() {
                (zone.states[idx], time, zone.transitions[n + 1].0)
            } else {
                (zone.states[idx], time, i64::max())
            }
        },
        None => {
            let state = match zone.states.find(|s| !s.1) {
                Some(i) => zone.states[i],
                _ => zone.states[0],
            };
            if zone.transitions.len() > 0 {
                (state, i64::min(), zone.transitions[0].0)
            } else {
                (state, i64::min(), i64::max())
            }
        },
    };

    date.offset = state.0;
    date.summer_time = state.1;

    let mut secs = 0;
    let mut res = date;

    while lo < hi {
        secs = lo / 2 + hi / 2;
        res = explode(zone, secs);
        match dt_cmp(&res, &date) {
            Ordering::Less => lo = secs + 1,
            Ordering::Greater => hi = secs,
            Ordering::Equal => break,
        }
    }

    (res, Time { seconds: secs, nanoseconds: 0 })
}

/// Find the largest transition such that the transition time expands to a DateTime which
/// is smaller than the given DateTime.
fn find_transition(zone: &Zone, date: &DateTime) -> Option<usize> {
    if zone.transitions.len() == 0 {
        None
    } else {
        // Find n such that n is the greatest transition index that expands to a DateTime
        // smaller or equal the normalized DateTime.
        match zone.transitions.find_binary(|t| dt_cmp(&explode(zone, t.0), &date)) {
            (Some(n), _) => Some(n),
            (_, 0) => None,
            (_, n) => Some(n - 1),
        }
    }
}

fn dt_cmp(one: &DateTime, two: &DateTime) -> Ordering {
    if one.year   < two.year   { return Ordering::Less;    }
    if one.year   > two.year   { return Ordering::Greater; }
    if one.month  < two.month  { return Ordering::Less;    }
    if one.month  > two.month  { return Ordering::Greater; }
    if one.day    < two.day    { return Ordering::Less;    }
    if one.day    > two.day    { return Ordering::Greater; }
    if one.hour   < two.hour   { return Ordering::Less;    }
    if one.hour   > two.hour   { return Ordering::Greater; }
    if one.minute < two.minute { return Ordering::Less;    }
    if one.minute > two.minute { return Ordering::Greater; }
    if one.second < two.second { return Ordering::Less;    }
    if one.second > two.second { return Ordering::Greater; }
    Ordering::Equal
}

/// Normalizes `second`, `minute`, `hour`, `day`, `month`, and `year`.
fn normalize(date: &mut DateTime) {
    let (mut min, mut sec) = (date.second as i64).div_rem(SECS_PER_MIN);
    if sec < 0 {
        min -= 1;
        sec += SECS_PER_MIN;
    }
    date.second = sec as i8;

    let (mut hour, mut min) = (date.minute as i64 + min).div_rem(MINS_PER_HOUR);
    if min < 0 {
        hour -= 1;
        min += MINS_PER_HOUR;
    }
    date.minute = min as i8;

    let (mut days, mut hour) = (date.hour as i64 + hour).div_rem(HOURS_PER_DAY);
    if hour < 0 {
        days -= 1;
        hour += HOURS_PER_DAY;
    }
    date.hour = hour as i8;

    let (mut years, mut month) = (date.month as i64 - 1).div_rem(MONTHS_PER_YEAR);
    if month < 0 {
        years -= 1;
        month += MONTHS_PER_YEAR;
    }
    let mut year = date.year + years;

    days += date.day as i64 - 1;
    while days < 0 {
        month -= 1;
        if month < 0 {
            year -= 1;
            month = MONTHS_PER_YEAR - 1;
        }
        days += DAYS_PER_MONTH[is_leap(year)][month as usize] as i64;
    }
    while days >= DAYS_PER_MONTH[is_leap(year)][month as usize] as i64 {
        days -= DAYS_PER_MONTH[is_leap(year)][month as usize] as i64;
        month += 1;
        if month >= MONTHS_PER_YEAR {
            year += 1;
            month = 0;
        }
    }

    date.year = year;
    date.month = month as i8 + 1;
    date.day = days as i8 + 1;
}
