use std::{thread, time};

const WAIT: time::Duration = time::Duration::from_secs(1);

fn parse_time(mut s: u128) -> String {

    // huge math
    let mut m = s / 60;
    let h = m / 60;
    m = m - ( h * 60 );
    s = s - ( h * 60 * 60 ) - ( m * 60);

    // singular or plural
    let h_str = if h == 1 {
        "hour"
    } else {
        "hours"
    };
    let m_str = if m == 1 {
        "minute"
    } else {
        "minutes"
    };
    let s_str = if s == 1 {
        "second"
    } else {
        "seconds"
    };

    // finally, format string
    if h > 0 {
        format!("{} {}, {} {}, {} {}", h, h_str, m, m_str, s, s_str)
    } else if m > 0 {
        format!("{} {}, {} {}", m, m_str, s, s_str)
    } else {
        format!("{} {}", s, s_str)
    }

}

fn main() {
    let mut num = 0u128;
    loop {
        println!("brb: {}", parse_time(num));
        thread::sleep(WAIT);
        num += 1;
    }
}
