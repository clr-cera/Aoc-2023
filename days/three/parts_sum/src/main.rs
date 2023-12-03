use std::io;



fn main() {
    let matrix = create_matrix();
    let mut sum = 0;
    for number in get_numbers(&matrix) {
        println!("{number}");
        sum += number;
    }
    println!("{sum}");
}

fn get_numbers(matrix: &Vec<Vec<char>>) -> Vec<u64>{
    let mut vector: Vec<u64> = Vec::new();
    for i in 0..matrix.len(){
        let mut is_part: bool = false;
        let mut num_chars: String = String::new();

        for j in 0..matrix[i].len(){
            let possible: Result<u64, _> = matrix[i][j].to_string().parse();
            match possible {
                Err(_) => {
                    if is_part {
                        vector.push(num_chars.parse().expect("how did a char enter here?"));
                    }
                    num_chars.clear();
                    is_part = false;
                },

                Ok(_) => {
                    num_chars.push(matrix[i][j]);
                    if is_connected(matrix, i, j) {
                        is_part = true;
                    }
                },

            }

        }
    }
    
    return vector;
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

fn is_connected(matrix: &Vec<Vec<char>>, i: usize, j:usize) -> bool {
    let mut nearby = String::new();
    let mut returnal = false;
    
    for m in i as isize-1..=i as isize+1 {
        for n in j as isize-1..=j as isize+1 {
            if m < matrix.len() as isize && m > 0 {
                if n < matrix[m as usize].len() as isize && n > 0 {
                    nearby.push(matrix[m as usize][n as usize]);
                }
            }
        }
    }
    for c in nearby.chars() {
        if !("1234567890.\n".contains(c)) {
            returnal = true;
            break;
        }
    }
    
    return returnal;
}

fn read_string() -> Option<String> {
    let mut str = String::new();
    
    match io::stdin().read_line(&mut str){
        Ok(0) => return None,
        _ => return Some(str),
    }    
}

