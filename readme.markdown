# datefmt

A silly program to adjust time strings using rust

# usage from stdin

    # example file format: [H]H:MM {AM|PM},DD
    datefmt < timedeltas.csv > newtimes.csv

# usage as a library

    fn add_ten_minutes() {
        let r = datefmt::add_minutes("9:13 AM", 10);
        assert_eq!(r, "9:23 AM");
    }

