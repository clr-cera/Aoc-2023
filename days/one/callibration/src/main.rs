use std::io;

fn get_number(string: String) -> u16{
    let mut digits = String::new();
   
    let mut first_digit: bool = true;

    let mut first: char = ' ';
    let mut last: char = ' ';
    
    for char in string.chars() {
        let possible: Result<u16,_>= char.to_string().parse();
        match possible {
            Err(_) => (),
            Ok(_) => {
                if first_digit {first = char; first_digit = false;};
                last = char;
            },
        }
    }

    digits.push(first); 
    digits.push(last);


    digits.parse().expect("You have somehow produced not a number!")
}

fn main() {
    let mut numbers: Vec<u16> = Vec::new();
    
    loop {
        let string = read_string();
        match string {
            Some(text) => numbers.push(get_number(text)),
            None => break,
        }
    }

    let mut sum = 0;
    for number in numbers {
        sum += number;
    }

    println!("{sum}");
}

fn read_string() -> Option<String> {
    let mut str = String::new();
    
    match io::stdin().read_line(&mut str){
        Ok(0) => return None,
        _ => return Some(str),
    }    
}

