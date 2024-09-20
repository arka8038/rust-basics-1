fn main() {
    println!("{}", is_even(2000000000));
    println!("{}", fibonacci_rec(5));
    println!("{}", fibonacci_iter(5));

    let name = String::from("Envelope");
    println!("Length of the string is {}", get_str_len(name))
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true
    }
    return false
}

fn fibonacci_rec(num: u32) -> u32 {
    if num == 0 {
        return 0
    }

    if num == 1 {
        return 1
    }

    return fibonacci_rec(num - 1) + fibonacci_rec(num -2)
}

fn fibonacci_iter(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;
    if num == 1 {
        return first;
    }

    if num == 2 {
        return second;
    }

    for _ in 0..(num - 1) {
        let temp = second;
        second = first + second;
        first = temp;
    }

    return second;
}

fn get_str_len(s: String) -> usize {
    return s.chars().count();
}