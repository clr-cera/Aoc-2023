use std::io;
const DEBUG: bool = true;

// This enum represents each spelled number possibility
enum Spelled{
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Spelled {
    fn parse(&mut self) -> char {
        match self {
            Self::One => '1',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
        }
    }
}

// This struct is used to walk through the spell of a number to check if the word is the same
struct SpelledWalker {
    number: Spelled,
    index: usize,
}

impl SpelledWalker {
    fn new(number: Spelled) -> Self{
        Self { number, index: 0 }
        
    }

    fn clear(&mut self) {
        self.index = 0
    }


    // This functtion walks a walker in one position if the letter is the same as the letter in the
    // position of the spelled version of the number, if it reaches the end of the word, it returns
    // true, if it sees it is not the same char , it clears the wallker
    fn in_walk(&mut self, string: &str, c: char) -> bool{
        if string.char_indices().find(|x| x.0 == self.index).expect(format!("{} does not have {} char", string, self.index).as_str()).1 == c {
            self.index+=1;
            if self.index == string.chars().count() {
                self.clear();
                return true;
            }
        }
        else {
            if c == string.chars().next().unwrap() {self.index = 1}
            else {self.index = 0}
                
        }
        
        return false;
    }

    fn walk(&mut self, c: char) -> bool{
        match self.number {
            Spelled::One => {
                return self.in_walk("one", c);
            },

            Spelled::Two => {
                return self.in_walk("two", c);
            },
            
            Spelled::Three => {
                return self.in_walk("three", c);
            },
            
            Spelled::Four => {
                return self.in_walk("four", c);
            },
            
            Spelled::Five => {
                return self.in_walk("five", c);
            },
            
            Spelled::Six => {
                return self.in_walk("six", c);
            },
            
            Spelled::Seven => {
                return self.in_walk("seven", c);
            },
            Spelled::Eight => {
                return self.in_walk("eight", c);
            },
            Spelled::Nine => {
                return self.in_walk("nine", c);
            },
        }
    }
}

// This struct is used to check for a number in a string, receiving  each char of the string and
// returning a Some(char) if it detects a number
struct Parser {
    spells: Vec<SpelledWalker>,
}

impl Parser {
    fn new() -> Self{
        let spells = vec![
            SpelledWalker::new(Spelled::One), 
            SpelledWalker::new(Spelled::Two), 
            SpelledWalker::new(Spelled::Three), 
            SpelledWalker::new(Spelled::Four), 
            SpelledWalker::new(Spelled::Five),
            SpelledWalker::new(Spelled::Six),
            SpelledWalker::new(Spelled::Seven),
            SpelledWalker::new(Spelled::Eight),
            SpelledWalker::new(Spelled::Nine),
        ];
        Parser {spells}
    }

    fn parse(&mut self, c: char) -> Option<char> {
        let possible_digit: Result<u16, _> = c.to_string().parse();
        
        match possible_digit {
            Ok(_) => {
                for walker in &mut self.spells {walker.clear()}
                if DEBUG {println!("digit parsed is {c}");}
                return Some(c)
            },
            Err(_) => (),
        };

        let mut returnal: Option<char> = None;

        for walker in &mut self.spells {
            if walker.walk(c) {
                if DEBUG {println!("walker returned {}", walker.number.parse());}
                returnal =  Some(walker.number.parse())
            }
        }

        returnal
    }
}

fn get_number(string: String) -> u16{
    let mut digits = String::new();
   
    let mut first_digit: bool = true;

    let mut first: char = ' ';
    let mut last: char = ' ';
    
    let mut parser = Parser::new();

    for char in string.chars() {
        let possible: Option<char>= parser.parse(char);
        match possible {
            None => (),
            Some(c) => {
                if first_digit {first = c; first_digit = false;};
                last = c;
            },
        }
    }

    if DEBUG {
        println!("These are the digits: first = {first}, last = {last}");
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
