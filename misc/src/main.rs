use std::time::{SystemTime, UNIX_EPOCH};

fn get_nstime() -> u64 {
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", dur);
    dur.as_secs() << 30 | dur.subsec_nanos() as u64
}

pub fn main(){
    println!("{}", get_nstime() )
}