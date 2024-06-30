

/**
*   Fibonacci fib(n) = fib(n-1) + fib(n-2)
*   fib(0) = 0
*   fib(1) = 1
*
*   0 .. fib_number as k
*       fib(k) = fib(k-1) + fib (k-2)
*
*       difference is we can memoize and go from two to a much larger number. 
*/
use std::io;
use std::collections::HashMap;

fn fib(n: &i32)->u64{
    let mut arr = HashMap::new();
   
    {
        for k in 1..n-1{
            _fib(&k, &mut arr);
        }
    }
    

    _fib(n, &mut arr)

}

fn _fib(n: &i32, arr: &mut HashMap<i32,u64>) -> u64{
    match n {
        0 => 0,
        1 => 1,
        _ => {
            if arr.contains_key(n){
                arr[n]
            }
            else{
                let fib_n_1 = _fib(&(n-1), arr);
                let fib_n_2=_fib(&(n-2), arr); 
                let new_entry = fib_n_1 + fib_n_2;
                arr.insert(*n, new_entry);
                new_entry
            }
        }
    }
}
    
fn main() {
    let mut fib_number = String::new();
    println!("CHOOSE YOUR NUMBER:");
    io::stdin().read_line(&mut fib_number).unwrap();

    let result = fib_number.trim().parse::<i32>();
   
    match result {
        Ok(val) => {
            if val > 93{
                println!("Value is too large and would cause overflow. stay under 93.")
            }
            else{
                let fib_of_val = fib(&val);
                println!("Fib({val}) = {fib_of_val}");
            }
            
        },
        Err(err) =>{
            println!("No Fib for you! {err}");
        }
    }
}
