// FIRST PART
/*fn find_xmas_word_str(array: &Vec<&str>, word: &str, opposite_word: &str) -> usize {
    let mut count = 0;
    for i in 0..array.len() {
        let row = &array[i];
        //a row can contain multiple times the word
        let mut j = 0;
        while j < row.len() {
            if row[j..].starts_with(word) || row[j..].starts_with(opposite_word) {
                count += 1;
                j += word.len() - 1;
            } else {
                j += 1;
            }
        }
    }
    count
}

fn find_xmas_word(array: &Vec<String>, word: &str, opposite_word: &str) -> usize {
    let mut count = 0;
    for i in 0..array.len() {
        let row = &array[i];
        //a row can contain multiple times the word
        let mut j = 0;
        while j < row.len() {
            if row[j..].starts_with(word) || row[j..].starts_with(opposite_word) {
                count += 1;
                j += word.len() - 1;
            } else {
                j += 1;
            }
        }
    }
    count
}

fn remove_longest_diagonal(array: &Vec<String>) -> Vec<String> {
    let mut longest_diagonal = String::new();
    for i in 0..array.len() {
        if array[i].len() > longest_diagonal.len() {
            longest_diagonal = array[i].clone();
        }
    }
    let mut new_array = Vec::new();
    for i in 0..array.len() {
        if array[i] != longest_diagonal {
            new_array.push(array[i].clone());
        }
    }
    new_array
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let rows = file.lines().collect::<Vec<&str>>();
    let row_len = rows[0].len();
    let columns_len = rows.len();

    let columns = (0..row_len)
        .map(|i| {
            rows.iter().map(|row| row.chars().nth(i).unwrap()).collect::<String>()
        })
        .collect::<Vec<String>>();

    let primary_diagonals1 = (0..row_len)
        .map(|i| {
            let mut diagonal = String::new();
            for j in 0..row_len {
                if i + j < row_len {
                    diagonal.push(rows[j].chars().nth(i + j).unwrap());
                }
            }
            diagonal
        })
        .collect::<Vec<String>>();

    let primary_diagonals2 = (0..columns_len)
        .map(|i| {
            let mut diagonal = String::new();
            for j in 0..columns_len {
                if i + j < columns_len {
                    diagonal.push(columns[j].chars().nth(i + j).unwrap());
                }
            }
            diagonal
        })
        .collect::<Vec<String>>();

    let primary_diagonals2_new = remove_longest_diagonal(&primary_diagonals2);

    let secondary_diagonals1 = (0..row_len)
        .map(|i| {
            let mut diagonal = String::new();
            for j in 0..row_len {
                if i + j < row_len {
                    diagonal.push(rows[j].chars().nth(row_len - i - j - 1).unwrap());
                }
            }
            diagonal
        })
        .collect::<Vec<String>>();

    // create all the secondary diagonals starting from the top right edge. Thengoing down on the last column, add the other secondary diagonals in order to arrive to have as last element the bottom right edge element
    let secondary_diagonals2 = (0..columns_len)
        .map(|i| {
            let mut diagonal = String::new();
            for j in 0..columns_len {
                if i + j < columns_len {
                    diagonal.push(rows[i + j].chars().nth(row_len - j - 1).unwrap());
                }
            }
            diagonal
        })
        .collect::<Vec<String>>();

    let secondary_diagonals2_new = remove_longest_diagonal(&secondary_diagonals2);

    
    let mut total = 0;
    total += find_xmas_word_str(&rows, "XMAS", "SAMX");
    total += find_xmas_word(&columns, "XMAS", "SAMX");
    total += find_xmas_word(&primary_diagonals1, "XMAS", "SAMX");
    total += find_xmas_word(&primary_diagonals2_new, "XMAS", "SAMX");
    total += find_xmas_word(&secondary_diagonals1, "XMAS", "SAMX");
    total += find_xmas_word(&secondary_diagonals2_new, "XMAS", "SAMX");

    println!("{}", total);
}*/

// SECOND PART
fn generate_three_x_three_matrixes(array: &Vec<&str>) -> Vec<Vec<String>> {
    let mut matrixes = Vec::new();
    for i in 0..array.len() - 2 {
        for j in 0..array[i].len() - 2 {
            let mut matrix = Vec::new();
            for k in 0..3 {
                matrix.push(array[i + k][j..j + 3].to_string());
            }
            matrixes.push(matrix);
        }
    }
    matrixes
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let rows = file.lines().collect::<Vec<&str>>();

    let matrixes = generate_three_x_three_matrixes(&rows);

    let mut count = 0;

    for matrix in matrixes {
        let mut primary_diagonal = String::new();
        let mut secondary_diagonal = String::new();

        for i in 0..3 {
            primary_diagonal.push(matrix[i].chars().nth(i).unwrap());
            secondary_diagonal.push(matrix[i].chars().nth(2 - i).unwrap());
        }

        if (primary_diagonal == "MAS" || primary_diagonal == "SAM") && (secondary_diagonal == "MAS" || secondary_diagonal == "SAM") {
            count += 1;
        }

    }

    println!("{}", count);
}