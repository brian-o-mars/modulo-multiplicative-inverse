fn main() {
    //Code in the next line serves as a test
    let b = multi_inv(189, 37);

    println!("Multiplcative inverse is {}", b);
}

//The function "multi_inv" takes two values i and n, and finds the modular multiplicative inverse of an integer i under modulo n.
// This relational is mathematical represented as ix being congruent to 1 (mod n), x here being the modular multiplicative inverse of i under modulo n.
// For more on congruence see here - https://en.wikipedia.org/wiki/Congruence_relation#Basic_example


fn multi_inv(i: i32, n: i32) -> i32 {

// The extended eucledian algorithm is a popular algorith used to get the modular multiplicative inverse of a number under a certain modulo.
// The "ex_euc_alg" function uses the extended eucledian algorithm to calculate the Bezout coefficients for integers x and y.
//The extended eucledian algorithm helps to get the Bezout coefficients s and t such that s*i+t*n = gcd(i,n). gcd refers to the greatest common divisor.
    fn ex_euc_alg(mut x: i32, mut y: i32) -> (i32, i32) {
    // The extended eucledian algorithm starts with initial values for r,s1,s2,t1 and t2. The values are initialised below
        let mut r = 1;
        let mut s1 = 1;
        let mut s2 = 0;
        let mut t1 = 0;
        let mut t2 = 1;
        
    // The extended eucledian algorithm is a repititive process that updates the values initialised above until r becomes 0
    // The following code makes the updates according to the algorithm until r becomes 0
        while r > 0 {
            let q = x / y;
            r = x - q * y;
            let s3 = s1 - q * s2;
            let t3 = t1 - q * t2;

            if r > 0 {
                x = y;
                y = r;
                s1 = s2;
                s2 = s3;
                t1 = t2;
                t2 = t3;
            }
        }
        // The "ex_euc_alg" function then returns |y| and t2 to the main "multi_inv" function
        return (y.abs(), t2);
    }
    //In using the extended eucledian algorithm the modulo and integer positions has to be switched
    //Assign |y| to a variable called gcd and t2 to a variable called t
    let (gcd, t) = ex_euc_alg(n, i);
    
    //Important to note that modulo multiplicative inverses doesn't exist for all integers under certain modulo. 
    //If gcd(i, n) isn't 1, modulo multiplicative inverse doesn't exist for that integer 1 under modulo n and this code just returns 0 instead.
    if gcd == 1 {
        // rem_euclid here does the same function as % in languages like python. % in rust sometimes produces unwanted results especially when the numerator is negative. 
        // This is futher discuessed in this reddit thread - https://www.reddit.com/r/rust/comments/r1rmv5/rust_says_207_6_when_it_is_1/
        return t.rem_euclid(n);
    } else {
        return 0;
    }
}



//I took some code inspiration from https://www.extendedeuclideanalgorithm.com/code.php
