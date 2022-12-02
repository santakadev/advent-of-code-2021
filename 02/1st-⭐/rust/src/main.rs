use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./src/input.txt").expect("Error reading the file");

    let mut horizontal_position = 0;
    let mut depth = 0;

    let mut command_dirs = HashMap::new();
    command_dirs.insert("forward", 1);
    command_dirs.insert("down", 1);
    command_dirs.insert("up", -1);

    for command in input.trim_end().split("\n") {
        let command_parts = command.split(" ").collect::<Vec<&str>>();
        let command_dir = command_parts[0];
        let command_value: i32 = command_parts[1].parse::<i32>().expect("Can't parse value");

        if command_dir == "forward" {
            horizontal_position += command_value;
        } else {
            depth += command_dirs[command_dir] * command_value; 
        }
    }

    println!("{}", horizontal_position * depth);
}
