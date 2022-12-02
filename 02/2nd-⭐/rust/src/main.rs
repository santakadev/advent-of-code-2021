use std::num::ParseIntError;
use std::str::FromStr;

enum Dir {
    Forward,
    Up,
    Down
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Dir::Forward),
            "up" => Ok(Dir::Up),
            "down" => Ok(Dir::Down),
            _ => Err(())
        }
    }
}

struct Command {
    dir: Dir,
    value: u32
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, value) = s.split_once(" ").unwrap();

        Ok(Command { dir: dir.parse::<Dir>().unwrap(), value: value.parse::<u32>().unwrap() })
    }
}

struct Submarine {
    position: PositionDiff,
    aim: i32
}

impl Submarine {
    fn initial() -> Self {
        Submarine { position: PositionDiff::initial(), aim: 0 }
    }

    fn apply(&self, command: Command) -> Submarine {
        return match command.dir {
            Dir::Forward => self.forward(command.value),
            Dir::Up => self.add_aim((-1) * command.value as i32),
            Dir::Down => self.add_aim(command.value as i32)
        };
    }

    fn forward(&self, value: u32) -> Submarine {
       let horizontal = self.position.horizontal + value;
       let depth = self.position.depth + self.aim * value as i32;
       let position = PositionDiff { horizontal, depth };

       Submarine { position, aim: self.aim }
    }

    fn add_aim(&self, value: i32) -> Submarine {
        let position = PositionDiff { horizontal: self.position.horizontal, depth: self.position.depth };
        Submarine { position, aim: self.aim + value }
    }
}

struct PositionDiff {
    horizontal: u32,
    depth: i32
}

impl PositionDiff {
    fn initial() -> Self {
        Self { horizontal: 0, depth: 0 }
    }

    fn mul(&self) -> u32 {
        return self.horizontal * self.depth as u32;
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input.txt").expect("Error reading the file");

    let mut submarine = Submarine::initial();

    let commands = input
        .trim_end()
        .split("\n")
        .into_iter()
        .map(|s| s.parse::<Command>().unwrap());

    for c in commands {
        submarine = submarine.apply(c);
    }

    println!("{}", submarine.position.mul());
}
