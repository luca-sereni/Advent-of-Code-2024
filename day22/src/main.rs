use std::collections::{HashMap, HashSet};

const NUM_ITERATIONS: usize = 2000;

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

fn compute_sum(secret_numbers: &Vec<usize>, is_part1: bool) -> usize {
    let mut cache: HashMap<usize, usize> = HashMap::new();
    if is_part1 {
        let mut sum = 0;
        for &number in secret_numbers {
            let mut secret_number = number;
            for _ in 0..NUM_ITERATIONS {
                secret_number = apply_modifications(secret_number, &mut cache);
            }
            sum += secret_number;
        }
        return sum;
    } else {
        let mut sequences_cache: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();
        for &number in secret_numbers {
            let mut secret_number = number;
            let mut secret_numbers_vector: Vec<usize> = vec![number];
            for _ in 0..NUM_ITERATIONS {
                secret_number = apply_modifications(secret_number as usize, &mut cache);
                secret_numbers_vector.push(secret_number);
            }

            let mut sequence: (isize, isize, isize, isize) = (0, 0, 0, 0);
            let mut sequences_set: HashSet<(isize, isize, isize, isize)> = HashSet::new();
            for i in 0..secret_numbers_vector.len() - 4 {
                sequence.0 = (((secret_numbers_vector[i + 1] % 10) as isize) - ((secret_numbers_vector[i] % 10) as isize)) as isize;
                sequence.1 = (((secret_numbers_vector[i + 2] % 10) as isize) - ((secret_numbers_vector[i + 1] % 10) as isize)) as isize;
                sequence.2 = (((secret_numbers_vector[i + 3] % 10) as isize) - ((secret_numbers_vector[i + 2] % 10) as isize)) as isize;
                sequence.3 = (((secret_numbers_vector[i + 4] % 10) as isize) - ((secret_numbers_vector[i + 3] % 10) as isize)) as isize;
                if !sequences_set.contains(&sequence) {
                    if let Some(&cached_result) = sequences_cache.get(&sequence) {
                        let price = (secret_numbers_vector[i + 4] % 10) as usize;
                        sequences_cache.insert(sequence, cached_result + price);
                    } else {
                        let price = secret_numbers_vector[i + 4] % 10;
                        sequences_cache.insert(sequence, price as usize);
                    }
                    sequences_set.insert(sequence);
                }
            }
        }

        return *sequences_cache.values().max().unwrap();
    }
}

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
    let sum = compute_sum(&initial_secret_numbers, true);
    println!("{}", sum);

    // PART 2
    let num_items_sold = compute_sum(&initial_secret_numbers, false);
    println!("{}", num_items_sold);
}
