mod parser;

fn main() {
    let arguments = std::env::args().collect::<Vec<String>>();

    if arguments.len() < 1 {
        println!("Please provide a file name");
        std::process::exit(1);
    }

    let calculation = arguments[1].clone();

    let mut parser = parser::Parser::new(calculation);
    let result = parser.parse();

    match result.op {
        '+' => println!("{}", result.lhs + result.rhs),
        '-' => println!("{}", result.lhs - result.rhs),
        '*' => println!("{}", result.lhs * result.rhs),
        '/' => println!("{}", result.lhs / result.rhs),
        _ => println!("Unknown operator"),
    }

}
