fn main() {
    let input = include_str!("./input1.txt").split("\n");
    let mut sum = 0;
    for line in input {
        // The split will include one empty line, in that case I want to continue
        if line.len() == 0 {
            continue;
        }
        let mut first_num_as_char: Option<char> = None;
        let mut last_num_as_char: Option<char> = None;

        for character in line.chars() {
            // Check if the character is an intiger
            let digit = character.to_digit(10);
            if digit == None {
                continue;
            }
            if first_num_as_char.is_none() {
                first_num_as_char = Some(character);
            } else {
                last_num_as_char = Some(character);
            }
        }
        // This check is just in case if there is no digits in the given line
        if first_num_as_char.is_none() && last_num_as_char.is_none() {
            continue;
        }
        // If there was only one digit in the line, that first digit is also the last
        if last_num_as_char.is_none() {
            last_num_as_char = first_num_as_char;
        }
        sum += format!(
            "{}{}",
            first_num_as_char.unwrap(),
            last_num_as_char.unwrap()
        )
        .parse::<u32>()
        .unwrap();
    }
    println!("Answer for this puzzle is: {}", sum);
}
