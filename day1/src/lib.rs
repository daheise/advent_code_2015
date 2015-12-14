
use std::io::{self, Read};

pub fn get_floor_string() -> String{
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();
    return buffer;

}

pub fn calculate_floor(instr: &str) -> isize {
    //This is a stupid way to do this but I can't figure out an easier way
    let v_up:Vec<&str> = instr.split('(').collect();
    let v_down:Vec<&str>   = instr.split(')').collect();
    let retval:isize = (v_up.len() as isize) - (v_down.len() as isize);
    retval
}

#[test]
fn it_works() {
  let instr = get_floor_string();
  let floor = calculate_floor(&instr);
  println!("{}", floor);
}

