use std::env;

mod numbers;
mod ascii;

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
                println!("percent({}, {}) = {}", 200, 20, numbers::percent(200, 20));
                println!("percent({}, 100 - {}) = {}", 200, 20, numbers::percent(200, 100 - 20));
                
                println!("min({}, {}) = {}", 3, -5, numbers::min(3, -5));
                println!("min2({}, {}) = {}", "foo", "bar", numbers::min2("foo", "bar"));
                // println!("min2({}, {}) = {}", 2.7182, 3.1415, numbers::min2(2.7182, 3.1415));
                println!("min3({}, {}, {}) = {}", "foo", "bar", "baz", numbers::min3("foo", "bar", "baz"));
                println!("partial_min2({}, {}) = {:?}", f64::NAN, 3.1415, numbers::partial_min2(&f64::NAN, &3.1415));
                println!("partial_min2({}, {}) = {:?}", 2.7182, 3.1415, numbers::partial_min2(&2.7182, &3.1415));

                println!("gdc({}, {}) = {}", 140, 96, numbers::gdc(140, 96));

                println!("sqrt({}) = {}", 0.0, numbers::sqrt(0.0));
                println!("sqrt({}) = {}", 0.000002, numbers::sqrt(0.000002));
                println!("sqrt({}) = {}", 1.0, numbers::sqrt(1.0));
                println!("sqrt({}) = {}", 2.0, numbers::sqrt(2.0));
                println!("sqrt({}) = {}", 3.0, numbers::sqrt(3.0));
                println!("sqrt({}) = {}", 4.0, numbers::sqrt(4.0));

                use std::f64::consts::PI;

                println!("sin({}) = {}", 0.0, numbers::sin(0.0));
                println!("sin({}) = {}", PI / 2.0, numbers::sin(PI / 2.0));
                println!("sin({}) = {}", PI / 6.0, numbers::sin(PI / 6.0));
                println!("sin({}) = {}", -PI / 6.0, numbers::sin(-PI / 6.0));

                println!("sin({}) = {}", 1.5 * PI, numbers::sin(1.5 * PI));
            },
            "ascii" => {
                println!("is_digit('{}') = {}", 'D', ascii::is_digit(b'D'));
                println!("is_digit('{}') = {}", '3', ascii::is_digit(b'3'));
                println!("is_lower('{}') = {}", 't', ascii::is_lower(b't'));
                println!("is_lower('{}') = {}", 'T', ascii::is_lower(b'T'));
                println!("is_upper('{}') = {}", 't', ascii::is_upper(b't'));
                println!("is_upper('{}') = {}", 'T', ascii::is_upper(b'T'));
                println!("is_letter('{}') = {}", 'D', ascii::is_letter(b'D'));
                println!("is_letter('{}') = {}", 'd', ascii::is_letter(b'd'));
                println!("is_letter('{}') = {}", '3', ascii::is_letter(b'3'));

                println!("to_upper('{}') = '{}'", 'F', char::from(ascii::to_upper(b'F')));
                println!("to_upper('{}') = '{}'", 'f', char::from(ascii::to_upper(b'f')));
                println!("to_upper('{}') = '{}'", '5', char::from(ascii::to_upper(b'5')));
                println!("to_upper('{}') = '{}'", ',', char::from(ascii::to_upper(b',')));
                println!("to_lower('{}') = '{}'", 'F', char::from(ascii::to_lower(b'F')));
                println!("to_lower('{}') = '{}'", 'f', char::from(ascii::to_lower(b'f')));

                println!("parse_32(\"{}\") = {:?}", "12345", ascii::parse_u32(b"12345"));
                println!("parse_32(\"{}\") = {:?}", "00027", ascii::parse_u32(b"00027"));
                println!("parse_32(\"{}\") = {:?}", "123ab", ascii::parse_u32(b"123ab"));
                println!("parse_32(\"{}\") = {:?}", "abcde", ascii::parse_u32(b"abcde"));
            },
            _ => {
                println!("Unrecognized parameter. Re-run application without arguments to help.");
            }
        }
    }
}
