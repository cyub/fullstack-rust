pub fn say_hello() {
    println!("Hello, world!");
}

pub fn print() {
    let numbers: [u8; 5] = [1, 2, 3, 4, 5];
    //let () = numbers;
    for n in numbers.iter() {
        println!("{}", n);
    }

    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("{}", n);
    }

    let numbers = [1, 2, 3, 4, 5];
    output_sequence(numbers);

    let numbers = vec![1, 2, 3, 4, 5];
    let another_numbers = numbers.clone();
    output_sequence_from_vec(numbers);

    // output_sequence_from_vec(numbers);
    output_sequence_from_vec_by_borrow(&another_numbers);
    output_sequence_from_vec_by_borrow(&another_numbers);

    let vector_numbers = vec![1, 2, 3, 4, 5];
    output_sequence_normalizing(&vector_numbers);
    let array_numbers = [1, 2, 3, 4, 5];
    output_sequence_normalizing(&array_numbers)
}

fn output_sequence(numbers: [u8; 5]) {
    for n in numbers.iter() {
        println!("{}", n);
    }
}

fn output_sequence_from_vec(numbers: Vec<u8>) {
    for n in numbers {
        println!("{}", n);
    }
}

fn output_sequence_from_vec_by_borrow(numbers: &Vec<u8>) {
    for n in numbers {
        println!("{}", n);
    }
}

fn output_sequence_normalizing(numbers: &[u8]) {
    for n in numbers {
        println!("{}", n);
    }
}

pub fn print_numbers(limit: u8) {
    let numbers = generate_sequence(limit);
    output_sequence_normalizing(&numbers);

    let numbers = generate_sequence_by_collection(limit);
    output_sequence_normalizing(&numbers);
}

fn generate_sequence(limit: u8) -> Vec<u8> {
    let mut numbers = Vec::new();
    for n in 1..=limit {
        numbers.push(n);
    }
    numbers
}

fn generate_sequence_by_collection(limit: u8) -> Vec<u8> {
    (1..=limit).collect()

    // Same effect
    // (1..=limit).collect::<Vec<u8>>()
}

#[test]
fn generate_sequence_should_work() {
    let result = generate_sequence(5);
    assert_eq!(result, &[1, 2, 3, 4, 5]);
}
