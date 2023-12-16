use std::io::{self, BufRead};
use std::fs::File;


fn main() -> io::Result<()>{
    let path = "input.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines(){
        let line = line?;
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

        if let(Some(&first_digit), Some(&last_digit)) = (digits.first(), digits.last()) {
            let number = format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
            sum += number;
        }
    }
    println!("{}", sum);
    Ok(())
}
