use std::io;
use std::f64::consts::PI;

fn main() {
    println!("How many didgits of pi?");
    let mut input = String::new();
    io::stdin().read_line( &mut input).unwrap();

    println!("{input}");
    let digit_count = input.trim().parse::<u32>();
    
    match digit_count {
        Ok(val) => {
            println!("{val}");
            let pistr = format!("{PI}");
            let formatted: String = pistr.chars().take((val + 2) as usize).collect();
            println!("{formatted}");
            
        },
        Err(err) =>{
            panic!("Input must be number. {err}")
        }
    }
}

// close enough, want to move on to more intersting things