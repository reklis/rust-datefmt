extern crate datefmt;

fn main() {
    let stdin = std::io::stdin();
    let mut linebuffer = String::new();

    loop {
        match stdin.read_line(&mut linebuffer) {
            Ok(i) => {
                if 0 == i {
                    break;
                }

                let mut tokens = linebuffer.split(',');
                let time = tokens.next().unwrap();
                let delta = tokens.next().unwrap().trim().parse::<i32>()
                    .expect("failed to parse time delta");
                let adjusted_time = datefmt::add_minutes(time, delta);
                println!("{}", adjusted_time);
            },
            Err(e) => { panic!(e); }
        }
        linebuffer.clear();
    }
}
