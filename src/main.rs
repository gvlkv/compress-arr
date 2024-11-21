use std::iter::from_fn;

use rand::prelude::*;

mod ordered;
mod unordered;
mod util;

use unordered::*;

fn sort<T: Clone + Ord>(input: &[T]) -> Vec<T> {
    let mut input = input.to_vec();
    input.sort();
    input
}

fn main() {
    let mut rng = rand::thread_rng();
    fn test(input: &[i32]) {
        println!(
            "  IN: {:?} {}",
            input.iter().take(50).collect::<Vec<_>>(),
            if input.len() > 50 { "..." } else { "" }
        );
        let result = encode(input);
        println!("  OUT: {:}", String::from_utf8_lossy(&result));
        assert_eq!(sort(&decode(&result)), sort(input));
        println!(
            "  len(IN): {}; len(OUT): {}; compress ratio: {}",
            input.len(),
            result.len(),
            input.len() as f32 / result.len() as f32
        );
        println!();
    }
    println!("Случайные 1..=300, 50 чисел.");
    let input = from_fn(|| Some(rng.gen_range(1..=300)))
        .take(50)
        .collect::<Vec<_>>();
    test(&input);
    println!("Случайные 1..=300, 100 чисел.");
    let input = from_fn(|| Some(rng.gen_range(1..=300)))
        .take(100)
        .collect::<Vec<_>>();
    test(&input);
    println!("Случайные 1..=300, 500 чисел.");
    let input = from_fn(|| Some(rng.gen_range(1..=300)))
        .take(500)
        .collect::<Vec<_>>();
    test(&input);
    println!("Случайные 1..=300, 1000 чисел.");
    let input = from_fn(|| Some(rng.gen_range(1..=300)))
        .take(1000)
        .collect::<Vec<_>>();
    test(&input);
    println!("Все числа из 1 знака.");
    let input: Vec<i32> = (1..10).collect();
    test(&input);
    println!("Все числа из 2 знаков.");
    let input: Vec<i32> = (10..100).collect();
    test(&input);
    println!("Все числа 1..=300 из 3 знаков.");
    let input: Vec<i32> = (100..301).collect();
    test(&input);
    println!("Все числа 1..=10 по 3 раза, всего чисел 30.");
    let mut input: Vec<i32> = (1..=10).flat_map(|x| [x, x, x]).collect();
    input.shuffle(&mut rng);
    test(&input);
    println!("Все числа 1..=300 по 3 раза, всего чисел 900.");
    let mut input: Vec<i32> = (1..=300).flat_map(|x| [x, x, x]).collect();
    input.shuffle(&mut rng);
    test(&input);
}
