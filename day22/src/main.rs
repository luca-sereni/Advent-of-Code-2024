use std::collections::HashMap;

fn apply_modifications(secret_number: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(&cached_result) = cache.get(&secret_number) {
        return cached_result;
    }
    let mut result = secret_number;

    let n1 = secret_number * 64;
    result = n1 ^ result;
    if result >= 16777216 {
        result = result % 16777216;
    }
    let n2 = result / 32;
    result = n2 ^ result;
    if result >= 16777216 {
        result = result % 16777216;
    }

    let n3 = result * 2048;
    result = n3 ^ result;
    if result >= 16777216 {
        result = result % 16777216;
    }

    cache.insert(secret_number, result);

    result
}

fn compute_sum(secret_numbers: &Vec<usize>) -> usize {
    let mut sum = 0;
    let mut cache: HashMap<usize, usize> = HashMap::new();
    for &number in secret_numbers {
        let mut secret_number = number;
        for _ in 0..NUM_ITERATIONS {
            secret_number = apply_modifications(secret_number, &mut cache);
        }
        sum += secret_number;
    }
    sum
}

const NUM_ITERATIONS: usize = 2000;

fn main() {
    let input = include_str!("../input.txt");

    let mut lines = input.lines();
    let mut initial_secret_numbers: Vec<usize> = Vec::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let number: usize = line.parse().unwrap();
        initial_secret_numbers.push(number);
    }

    // PART 1
    let sum = compute_sum(&initial_secret_numbers);
    println!("{}", sum);
}
