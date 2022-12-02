fn main() {
    let input = std::fs::read_to_string("./src/input.txt").expect("Error while reading the file");

    let numbers = input.trim_end().split("\n").collect::<Vec<&str>>();
    let length = numbers[0].len();
    let num_numbers = numbers.len();
    let mut occurrences = vec![0; length];
    for s in numbers {
        for (i, c) in s.chars().into_iter().enumerate() {
            occurrences[i] += if c == '1' { 1 } else { 0 };
        }
    }

    println!("{:?}", occurrences);

    let mut gamma: u32 = 0;
    let mut mask = 0;
    for (i, occ) in occurrences.iter().enumerate() {
        if occ > &(num_numbers / 2) {
            gamma = gamma | (0b1 << length - 1 - i);
        } 
        mask = mask | (0b1 << length - 1 - i);
    }



    let epsilon = gamma ^ mask;

    println!("{:?}", gamma);
    println!("{:?}", epsilon);
    println!("{}", gamma * epsilon);
}
