use std::io;

//Задача №1: Нахождение максимальной суммы подряд идущих элементов

fn max_sum(){
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let n: usize = input.trim().parse().expect("!");
    input.clear();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let array: Vec<isize> = input
        .trim()  
        .split_whitespace() 
        .map(|s| s.parse().expect("Expected a number")) 
        .collect();  

    let mut max_sum = array[0];
    let mut current_sum = array[0];
    for i in 1..n{
        current_sum += &array[i];        
        if current_sum > max_sum{
            max_sum = current_sum;    
        }
        if current_sum < 0{
            current_sum = 0;    
        }
    }

    println!("{}", max_sum)
}

fn max_sum_with_multiplication(){
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let n: usize = input.trim().parse().expect("!");
    input.clear();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let array: Vec<isize> = input
        .trim()  
        .split_whitespace() 
        .map(|s| s.parse().expect("Expected a number")) 
        .collect();  

    
    let mut max_sum_no_flip = array[0];
    let mut max_sum_with_flip = -array[0]; 
    let mut current_sum_no_flip = array[0]; 
    let mut current_sum_with_flip = -array[0]; 

    let mut overall_max = array[0]; 

    for i in 1..n {
        let num = array[i];
        current_sum_no_flip = current_sum_no_flip.max(0) + num;
        current_sum_with_flip = (current_sum_with_flip.max(0) + num).max(current_sum_no_flip - 2 * num);
        max_sum_no_flip = max_sum_no_flip.max(current_sum_no_flip);
        max_sum_with_flip = max_sum_with_flip.max(current_sum_with_flip);
        overall_max = overall_max.max(max_sum_no_flip).max(max_sum_with_flip);
    }

    println!("{}", overall_max);
}

fn main() {
    max_sum_with_multiplication();
}

