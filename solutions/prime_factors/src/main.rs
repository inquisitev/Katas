use std::io::stdin;
use primes::{Sieve, PrimeSet, is_prime};
// display all prime factors of the input
fn main() {
    let mut buf = String::new();
    
    println!("Enter number for prime factorization: ");
    stdin().read_line(&mut buf).unwrap();
    let val = buf.trim().parse::<u64>();

    match val{
        Ok(val_to_factor)=>{

            let factors = make_factors(val_to_factor);
            let formatted_factors = format!("{:?}", factors);
            println!("The prime factors of {val_to_factor} are {formatted_factors}")
        }
        
        Err(err) => {println!("Error: {err}")}
    } 
    
}

fn make_factors(val: u64) -> Vec<u64>{
    let mut temp_val = val;
    let mut factors: Vec<u64> = vec!();

    let mut pset = Sieve::new();
    while !is_prime(temp_val){
        for n in pset.iter(){
            if temp_val % n == 0{
                temp_val /=  n;
                factors.push(n);
                break;
            }
        }

    }

    factors.push(temp_val);

    factors
}

#[test]
fn test_factors_of_12() {
    let actual = make_factors(12);
    let expected: Vec<u64> = [2,2,3].to_vec();
    assert_eq!(expected, actual);
}

#[test]
fn test_factors_of_32() {
    let actual = make_factors(32);
    let expected: Vec<u64> = [2, 2, 2, 2, 2].to_vec();
    assert_eq!(expected, actual);
}
