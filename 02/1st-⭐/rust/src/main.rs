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

impl Command {
    fn position_diff(&self) -> PositionDiff {
        let horizonal_position = match self.dir {
            Dir::Forward => self.value,
            Dir::Up => 0,
            Dir::Down => 0
        };

        let depth: i32 = match self.dir {
            Dir::Forward => 0,
            Dir::Up => (-1) * self.value as i32,
            Dir::Down => self.value as i32
        };

        PositionDiff { horizonal_position: horizonal_position, depth: depth }
    }
}

struct PositionDiff {
    horizonal_position: u32,
    depth: i32
}

impl PositionDiff {
    fn add(&self, other: PositionDiff) -> PositionDiff {
        PositionDiff { 
            horizonal_position: self.horizonal_position + other.horizonal_position,
            depth: self.depth + other.depth
        }
    }

    fn mul(&self) -> u32 {
        return self.horizonal_position * self.depth as u32;
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input.txt").expect("Error reading the file");

    let final_position = input
        .trim_end()
        .split("\n")
        .into_iter()
        .map(|s| s.parse::<Command>().unwrap())
        .map(|c| c.position_diff())
        .reduce(|pos1, pos2| pos1.add(pos2))
        .unwrap();

    println!("{}", final_position.mul());
}
