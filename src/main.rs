

fn factorial(n: u32) -> number_factorization::Factorization {
    // n*(n-1)*...*i / ( k*(k-1)*...*1 > upper_bound ?

    let mut f = number_factorization::get_factorization(2);

    for i in 3..n + 1 {
        let f1 = number_factorization::get_factorization(i);
        f = number_factorization::multiply(f, f1);
    }

    return f;
}

fn get_n_over_r(n: u32, r: u32) -> u128 {
    
    let a = factorial(n);
    let b = number_factorization::multiply( factorial(r), factorial(n-r));

    let c = number_factorization::division( a, b );

    let r = number_factorization::get_number(c);

    return r;

}

fn main() {
    assert!(1 == number_factorization::get_number(factorial(1)));
    assert!(2 == number_factorization::get_number(factorial(2)));
    assert!(6 == number_factorization::get_number(factorial(3)));
    assert!(get_n_over_r(23, 10) == 1144066);
    println!("Hello, world!");

    let mut k = 0;
    for n in 1..101 {
        for r in 1..n {
            if get_n_over_r(n, r) > 1000000 {
                k += 1;
            }
        }
    }
    println!("answer: {}", k);
}
