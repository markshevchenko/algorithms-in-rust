use std::env;

mod numbers;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: ");
        println!("  code chapter");
        println!();
        println!("Where chapter is:");
        println!("  numbers -- algorithms with numbers");
    } else {
        match args[1].as_str() {
            "numbers" => {
                println!("percent({}, {}) is {}", 200, 20, numbers::percent(200, 20));
                println!("percent({}, 100 - {}) is {}", 200, 20, numbers::percent(200, 100 - 20));
                println!("min({}, {}) is {}", 3, -5, numbers::min(3, -5));
                println!("min2({}, {}) is {}", "foo", "bar", numbers::min2("foo", "bar"));
                // println!("min2({}, {}) is {}", 2.7182, 3.1415, numbers::min2(2.7182, 3.1415));
                println!("min3({}, {}, {}) is {}", "foo", "bar", "baz", numbers::min3("foo", "bar", "baz"));
                println!("partial_min2({}, {}) is {:?}", f64::NAN, 3.1415, numbers::partial_min2(&f64::NAN, &3.1415));
                println!("partial_min2({}, {}) is {:?}", 2.7182, 3.1415, numbers::partial_min2(&2.7182, &3.1415));
            },
            _ => {
                println!("Unrecognized parameter. Re-run application without arguments to help.");
            }
        }
    }
}
