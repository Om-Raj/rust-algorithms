/**
* Calculate the Greatest Common Divisor (GCD) of two positive numbers.
*
* Author: Om-Raj
* Date: 7 April, 2023
* Algorithm: Stein's algorithm / Binary Euclidean algorithm 
* 
* @param a: u64 - a positive integer
* @param b: u64 - a positive integer
*
* @return u64 - the GCD of the two numbers a and b
*/

fn gcd(mut a: u64, mut b: u64) -> u64 {
    // If one of the numbers is zero, return the other number as the GCD
    if a == 0 { return b; }
    if b == 0 { return a; }

    // Compute the highest power of 2 that divides both a and b
    let shift = (a | b).trailing_zeros();

    // Divide a and b by this power of 2 and further divide to remove remaining factors of 2 in a
    a >>= a.trailing_zeros();
    b >>= shift;

    // Use the Euclidean algorithm to compute the GCD
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            let temp = a;
            a = b;
            b = temp;
        }
        b -= a;
        if b == 0 { break; }
    }

    // Multiply the GCD by the power of 2 that we divided out
    a << shift
}


// Creating a main() to call the gcd() function with demo values
fn main() {
    let n1: u64 = 9135622588;
    let n2: u64 = 5743963140;
    println!("Greatest Common Divisor(GCD) for {} and {} is {}", n1, n2, gcd(n1,n2));
}
