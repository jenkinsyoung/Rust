use std::io;
fn max_sum(){
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let n: usize = input.trim().parse().expect("!");

    let mut array = Vec::<isize>::with_capacity(n);
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    array = input2
        .trim()  
        .split_whitespace() 
        .map(|s| s.parse().expect("Expected a number")) 
        .collect();  
    println!("{:?}", array);

    
}

fn main() {
   max_sum();
}

