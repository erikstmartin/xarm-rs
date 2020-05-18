use hex;
use std::collections::HashMap;
use std::env;
use std::thread;
use xarm::Arm;

const DELAY_BETWEEN_COMMANDS: u32 = 1500;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: martian <serial_port> <phrase>");
        return;
    }

    let ports = serialport::available_ports().unwrap();
    let port_names: Vec<String> = ports.iter().map(|p| p.port_name.to_owned()).collect();
    if !port_names.contains(&args[1]) {
        println!(
            "Port \"{}\" not found! Available Ports: {:?}",
            args[2], port_names
        );
        return;
    }

    let port = serialport::open(&args[1]).unwrap();
    let mut arm = Arm::new(port);

    arm.reset();

    let mut lookup: HashMap<char, [u16; 6]> = HashMap::new();
    populate_lookup_table(&mut lookup);

    let chars: Vec<char> = hex::encode(&args[2]).to_uppercase().chars().collect();
    println!("{:?}", chars);

    for c in chars.iter() {
        touch_letter(&lookup, &mut arm, &c);
    }
}

fn touch_letter(lookup: &HashMap<char, [u16; 6]>, arm: &mut Arm, letter: &char) {
    if let Some(pos) = lookup.get(letter) {
        touch(arm, pos.to_owned());
    }
}

fn touch(arm: &mut Arm, pos: [u16; 6]) {
    // Rotate base first
    thread::sleep_ms(DELAY_BETWEEN_COMMANDS);
    arm.set_servo_position(6, pos[5], 1000);
    thread::sleep_ms(DELAY_BETWEEN_COMMANDS);

    arm.set_position(pos, 1000);
    thread::sleep_ms(DELAY_BETWEEN_COMMANDS);

    // Lift arm up first for transition
    if pos[4] > 500 {
        arm.set_servo_position(5, pos[4] - 200, 1000);
    } else {
        arm.set_servo_position(5, pos[4] + 200, 1000);
    }
}

// TODO: Inverse Kinamatics
fn populate_lookup_table(lookup_table: &mut HashMap<char, [u16; 6]>) {
    lookup_table.insert('A', [700, 634, 137, 627, 340, 520]);
    lookup_table.insert('B', [700, 634, 137, 627, 340, 425]);
    lookup_table.insert('C', [700, 634, 137, 627, 340, 344]);
    lookup_table.insert('D', [700, 634, 137, 627, 340, 256]);
    lookup_table.insert('E', [700, 634, 137, 627, 340, 188]);
    lookup_table.insert('F', [700, 634, 137, 627, 340, 100]);
    lookup_table.insert('0', [700, 634, 790, 368, 737, 780]);
    lookup_table.insert('1', [700, 634, 790, 368, 737, 700]);
    lookup_table.insert('2', [700, 634, 790, 368, 737, 612]);
    lookup_table.insert('3', [700, 634, 790, 368, 737, 520]);
    lookup_table.insert('4', [700, 634, 790, 368, 737, 420]);
    lookup_table.insert('5', [700, 634, 790, 368, 737, 330]);
    lookup_table.insert('6', [700, 634, 790, 368, 737, 240]);
    lookup_table.insert('7', [700, 634, 790, 368, 737, 150]);
    lookup_table.insert('8', [700, 634, 137, 627, 340, 855]);
    lookup_table.insert('9', [700, 634, 137, 627, 340, 760]);
    lookup_table.insert('?', [700, 634, 137, 627, 340, 679]);
    lookup_table.insert(' ', [700, 634, 137, 627, 340, 591]);
}
