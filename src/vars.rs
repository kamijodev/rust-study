pub mod sub_a;
pub mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module");
    // sub_a::func_a();
    // sub_b::func_b();
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);
    println!("Memory address of const is: {:p}", &MAX_POINTS);
}
