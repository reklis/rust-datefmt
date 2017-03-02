struct Time {
    hour: i32,
    minute: i32,
}

impl Time {
    fn from_string(date_string: &str) -> Time {
        // [H]H:MM {AM|PM}

        let separators: &[char] = &[':', ' '];
        let mut tokens = date_string.split(separators);

        let mut hours = tokens.next().unwrap().parse::<i32>()
            .expect("failed to parse hours");
        let minutes = tokens.next().unwrap().parse::<i32>()
            .expect("failed to parse minutes");

        let meridian = tokens.next().unwrap();

        if "PM" == meridian {
            hours += 12;
        }

        let t = Time { hour: hours, minute: minutes };
        t
    }

    fn to_string(&self) -> String {
        let mut s = String::new();

        if self.hour > 12 {
            let h = self.hour - 12;
            s.push_str(&h.to_string());
        } else {
            s.push_str(&self.hour.to_string());
        }

        s.push(':');
        if self.minute < 10 {
            s.push('0');
        }
        s.push_str(&self.minute.to_string());

        s.push(' ');
        s.push_str(if self.hour > 12 {"PM"} else {"AM"});

        s
    }
}

trait MutableTime {
    fn add_minutes(&mut self, minutes: i32);
}

impl MutableTime for Time {
    fn add_minutes(&mut self, minutes: i32) {
        let mut hour_delta = minutes / 60;
        loop {
            if hour_delta >= 24 {
                hour_delta -= 24;
            } else {
                break;
            }
        }
        self.hour += hour_delta;

        let minute_delta = minutes % 60;
        self.minute += minute_delta;
    }
}

pub fn add_minutes(time: &str, mins: i32) -> String {
    let mut t = Time::from_string(time);
    t.add_minutes(mins);
    t.to_string()
}

