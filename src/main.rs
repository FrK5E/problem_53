fn get_shortened_factorial( i: u32, j:u32  ) -> u128 {

    assert!( j<= i); 
    let mut acc:u128 = j as u128; 

    for k in j+1..i+1 { 
        acc *= k as u128;
    }

    acc


}



fn factorial( n: u32 ) -> number_factorization::Factorization {
   
   // n*(n-1)*...*i / ( k*(k-1)*...*1 > upper_bound ?

   let mut f : number_factorization::Factorization; 

   for i in 0..n+1 {

    let f1 = number_factorization::get_factorization(i);
    f = number_factorization::multiply( f, f1 );

   }


}


fn get_n_over_r( n: u32, r: u32 ) -> u128 { 

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

let mut k=0; 
    for n in 1..101 { 
        for r in 1..n { 
            if get_n_over_r(n,r )> 1000000 { 
k+=1;
            }
        }
    }
    println!( "answer: {}", k);
}
