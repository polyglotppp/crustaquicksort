use std::fs;

fn main() {
    let output = match fs::read_to_string("./bla") {
        Ok(content) => content
            .trim()
            .split("\n")
            .map(|a| a.parse::<i32>().unwrap())
            .collect::<Vec<i32>>(),
        Err(e) => {
            println!("could not read ./bla: {}", e);
            Vec::new()
        }
    };
    // println!("{:?}", quicksort(output));
}

fn quicksort(a: Vec<i32>) -> Vec<i32> {
    if a.len() <= 1 {
        return a;
    }

    let mut pivot = Vec::new();
    let mut lower = Vec::new();
    let mut higher = Vec::new();

    for num in a.iter() {
        let n = num.clone();
        match n {
            _ if pivot.len() == 0 => pivot.push(n),
            _ if n == a[0] => pivot.push(n),
            _ if n < a[0] => lower.push(n),
            _ if n > a[0] => higher.push(n),
            _ => println!("WAAAAAAAAAH! NEVER HAPPENS!"),
        }
    }

    return [quicksort(lower), pivot, quicksort(higher)].concat();
}
