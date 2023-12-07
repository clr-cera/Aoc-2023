use std::io;

const DEBUG: bool = false;


fn main() {
    let matrix = create_matrix();
    let mut sum = 0;
    for number in get_gears(&matrix) {
        if DEBUG{println!("{number}")};

        sum += number;
    }
    println!("{sum}");
}

fn create_matrix() -> Vec<Vec<char>>{
    let mut matrix: Vec<Vec<char>> = Vec::new();    
    loop {
        match read_string() {
            None => break,
            Some(text) => {
                let mut vector: Vec<char> = Vec::new();
                for c in &mut text.chars() {
                    vector.push(c);
                }
                matrix.push(vector);
            }
        }
    }
    return matrix;
}

fn get_gears(matrix: &Vec<Vec<char>>) -> Vec<u64>{
    let mut vector: Vec<u64> = Vec::new();
    
    for i in 0..matrix.len(){
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '*'{
                vector.push(gear_ratio(matrix,i,j));
            } 
        }
    }

    return vector;
}

fn gear_ratio(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> u64{
    let mut ratio: u64 = 1;
    let mut count: u16 = 0;
    let mut same_number = false;
    let i = i as isize;
    let j = j as isize;
    
    for m in i-1..=i+1 {
        for n in j-1..=j+1 {
            if m >= 0 && m < matrix.len() as isize {
                if n >= 0 && n < matrix[m as usize].len() as isize {
                    if n == j-1 {same_number = false;}
                    let possible: Result<u64, _> = matrix[m as usize][n as usize].to_string().parse(); 
                    match possible {
                        Err(_) => same_number = false,

                        Ok(_) => {
                            if same_number == false {
                                count+=1;
                                ratio *= get_number(matrix, m, n);
                                same_number = true;
                            }
                        }
                    }
                }
            }
        }
    }

    if count == 2 {return ratio;}
    else {return 0}
}

fn get_number(matrix: &Vec<Vec<char>>, i: isize, j: isize) -> u64 {
    let mut spelled_number = String::new();
    let line = &(matrix[i as usize]);

    spelled_number.push(line[j as usize]);

    // Get left
    let mut index = j-1;
    loop {
        if index < 0 {break}
        let possible: Result<u64, _> = line[index as usize].to_string().parse();
        match possible {
            Err(_) => break,
            Ok(_) => {
                spelled_number.insert(0, line[index as usize]);
                index -=1
            }
        }
    }
    
    // Get right
    let mut index = j+1; 
    loop {
        if index >= line.len() as isize {break}
        let possible: Result<u64, _> = line[index as usize].to_string().parse();
        match possible {
            Err(_) => break,
            Ok(_) => {
                spelled_number.push(line[index as usize]);
                index +=1
            }
        }
    }

    return spelled_number.parse().expect("Somehow this ended up not being a number!");
}

fn read_string() -> Option<String> {
    let mut str = String::new();
    
    match io::stdin().read_line(&mut str){
        Ok(0) => return None,
        _ => return Some(str),
    }    
}

