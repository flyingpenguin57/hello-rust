use rand::Rng;
use service::mod1;
use service::mod2;
use service::util;

fn main() {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(0..100);
    println!("Generated random number: {}", number);

    mod1::mod1_method_1("Alice");
    mod2::mod2_method_2();
    util::util_method_1();
}

