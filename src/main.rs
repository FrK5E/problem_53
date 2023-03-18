fn get_shortened_factorial( i: u32, j:u32  ) -> u64 {

    assert!( j<= i); 
    let mut acc:u64 = j as u64; 

    for k in j+1..i+1 { 
        acc *= k as u64;
    }

    acc

}


fn get_n_over_r( n: u32, r: u32 ) -> u64 { 

    let mut hi = r; 
    let mut low =  n-r;

    if hi<low {
        let temp = hi; 
        hi = low; 
        low = temp;   
    } 

    get_shortened_factorial(n, hi+1) / get_shortened_factorial( low, 1 ) 

}

fn main() {

    assert!( get_shortened_factorial(3, 1) == 6 );
    assert!( get_shortened_factorial(23, 22) == 23*22 );
    assert!( get_n_over_r(5, 3)==10);
    assert!( get_n_over_r(23, 10) == 1144066 );
    println!("Hello, world!");
}
