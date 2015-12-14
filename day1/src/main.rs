extern crate day1;

fn main() {
    let instr = day1::get_floor_string();
    let floor = day1::calculate_floor(&instr);
    println!("{}", floor);
}
