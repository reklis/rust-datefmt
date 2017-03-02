extern crate datefmt;

#[test]
#[should_panic]
fn bad_date() {
    datefmt::add_minutes("foo", 10);
}

#[test]
fn add_ten_minutes() {
    let r = datefmt::add_minutes("9:13 AM", 10);
    assert_eq!(r, "9:23 AM");
}

#[test]
fn add_one_hour() {
    let r = datefmt::add_minutes("9:13 AM", 60);
    assert_eq!(r, "10:13 AM");
}

#[test]
fn add_six_hours() {
    let r = datefmt::add_minutes("9:13 AM", 60 * 6);
    assert_eq!(r, "3:13 PM");
}

#[test]
fn add_one_day() {
    let r = datefmt::add_minutes("9:13 AM", 60 * 24);
    assert_eq!(r, "9:13 AM");
}

#[test]
fn add_one_day_two_hours_one_minute() {
    let r = datefmt::add_minutes("9:13 AM", 60 * 24 + 60 * 2 + 1);
    assert_eq!(r, "11:14 AM");
}

#[test]
fn add_one_hour_two_minutes() {
    let r = datefmt::add_minutes("9:13 AM", 62);
    assert_eq!(r, "10:15 AM");
}

#[test]
fn minus_ten_minutes() {
    let r = datefmt::add_minutes("9:13 AM", -10);
    assert_eq!(r, "9:03 AM");
}

#[test]
fn minus_one_hour() {
    let r = datefmt::add_minutes("9:13 AM", -60);
    assert_eq!(r, "8:13 AM");
}

