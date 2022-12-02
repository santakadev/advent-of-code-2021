enum Rating {
    Oxygen,
    CO2,
}

fn calc_rating(numbers: &Vec<String>, rating: &Rating, index: &usize) -> String {
    let num_count = &numbers.len();

    if *num_count == 1 {
        return numbers[0].clone();
    }

    let mut occurrences = 0;
    let mut one_numbers = vec![];
    let mut zero_numbers = vec![];
    for s in &numbers.to_vec() {
        let nth_char: char = s.chars().nth(*index).unwrap();
        if nth_char == '1' {
            occurrences += 1;
            one_numbers.push(s.clone());
        } else {
            zero_numbers.push(s.clone());
        }
    }

    
    let most_common_numbers = match rating {
        Rating::Oxygen => &one_numbers,
        Rating::CO2 => &zero_numbers
    };

    let less_common_numbers = match rating {
        Rating::Oxygen => &zero_numbers,
        Rating::CO2 => &one_numbers
    };
    

    if occurrences >= (*num_count + 1) / 2 {
        calc_rating(most_common_numbers, &rating, &(*index + 1))
    } else {
        calc_rating(less_common_numbers, &rating, &(*index + 1))
    }
}

fn bin_str_to_int(bin_str: &String) -> u32 {
    let len = bin_str.len();
    let mut num: u32 = 0;
    for (i, c) in bin_str.chars().enumerate() {
        if c == '1' {
            num = num | (0b1 << len - 1 - i)
        }
    }
    num
}

fn oxygen(numbers: &Vec<String>) -> String {
    return calc_rating(numbers, &Rating::Oxygen, &0);
}

fn co2(numbers: &Vec<String>) -> String {
    return calc_rating(numbers, &Rating::CO2, &0);
}

fn main() {
    let input = std::fs::read_to_string("./src/input.txt").expect("Error while reading the file");

    let numbers = input.trim_end().split("\n").map(|s| s.to_string()).collect::<Vec<String>>();

    let oxygen = oxygen(&numbers);
    let co2 = co2(&numbers);

    println!("{}", bin_str_to_int(&oxygen) * bin_str_to_int(&co2));
}
