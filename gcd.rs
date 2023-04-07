/**
* Calculate the Greatest Common Divisor (GCD) of two positive numbers.
*
* Author: Om-Raj
* Date: 7 April, 2023
*
* @param a: u64 - a positive integer
* @param b: u64 - a positive integer
*
* @return u64 - the GCD of the two numbers a and b
*/


fn gcd(mut a: u64, mut b: u64) -> u64
{
    // If one of the inputs is zero, return the other input as the GCD
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    
    // Initialize a factor to keep track of the common factors of 2
    let mut f : u64 = 1;

    // Continue looping until we find the GCD
    loop {
        // If a and b are equal, return the GCD multiplied by the factor
        if a == b {
            return a * f;
        }

        // If a is even and b is even, divide both by 2 and multiply the factor by 2
        if a & 1 == 0 {
            if b & 1 == 0 {
                a = a >> 1;
                b = b >> 1;
                f = f << 1;
                continue;
            }
            // If only a is even, divide a by 2
            a = a >> 1;
            continue;
        }

        // If a is odd and b is even, divide b by 2
        else {
            if b & 1 == 1 {
                // If a is greater than b, subtract b from a
                if a > b {
                    a = a - b;
                    continue;
                }
                // If b is greater than a, subtract a from b
                b = b - a;
                continue;
            }
            // If only b is even, divide b by 2
            b = b >> 1;
            continue;
        }
    }
}

// Creating a main() to call the gcd() function with demo values
fn main() {
    let n1: u64 = 9135622588;
    let n2: u64 = 5743963140;
    println!("Greatest Common Divisor(GCD) for {} and {} is {}", n1, n2, gcd(n1,n2));
}
