use std::io;

const RED: u64 = 12;
const GREEN: u64 = 13;
const BLUE: u64 = 14;
const DEBUG: bool = true;
const PART: u16 = 2;

#[derive(Debug)]
struct Game {
    id: u64,
    blue: u64,
    red: u64,
    green: u64,
}

enum Data {
    Id,
    Blue,
    Red,
    Green,
}

impl Game {
    fn new() -> Self {
        Self {id: 0, blue: 0, red: 0, green: 0}
    }

    fn parse(string: &mut String) -> Self{
        let mut returnal: Self = Self::new();
        let mut data: Data = Data::Id;
        let mut value: u64 = 0;
        let mut is_first: bool = true;

        for word in get_words(string) {
            match word {
                "Game" => data = Data::Id,
                
                "blue" => {
                    data = Data::Blue;
                    returnal.insert_info(value, &data);
                },

                "green" => {
                    data = Data::Green;
                    returnal.insert_info(value, &data);
                },

                "red" => {
                    data = Data::Red;
                    returnal.insert_info(value, &data);
                },
                
                _ => {
                    value = word.parse().expect("Why is this not a number?");
                    if is_first {
                        returnal.insert_info(value, &data);
                        is_first = false; 
                    }
                },
            }
        }

        if DEBUG{
            println!("{:?}", returnal);
        }

        return returnal;
    }

    fn insert_info(&mut self, number: u64, data: &Data) {
        match data {
            Data::Id => self.id = number,
            Data::Red => if self.red < number {self.red = number},
            Data::Blue => if self.blue < number {self.blue = number},
            Data::Green => if self.green < number {self.green = number},
        }
    }
}


fn get_games() -> Vec<Game>{
    let mut games: Vec<Game> = Vec::new();

    loop {
        let mut string: String;
        match read_string() {
            None => {break},
            Some(text) => string = text, 
        }

        games.push(Game::parse(&mut string));
    }

    return games;
}

fn main() {
    let games = get_games();
    let mut sum = 0;

    for game in games {
        if !(game.blue > BLUE || game.red > RED || game.green > GREEN) && PART == 1{
            sum += game.id;
        }
        else if PART == 2 {
            sum += game.red * game.blue * game.green; 
        }
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

fn get_words(string: &mut String) -> Vec<&str> {
    string.retain(|c| !r#"(),".;:'"#.contains(c));
    let vector: Vec<&str> = string.split_whitespace().collect();

    return vector;
}
