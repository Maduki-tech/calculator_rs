#[derive(Debug)]
struct Parser {
    pos: usize,
    input: String,
}

#[derive(Debug)]
struct Ast {
    op: char,
    lhs: i32,
    rhs: i32,
}

fn main() {
    let arguments = std::env::args().collect::<Vec<String>>();

    if arguments.len() < 1 {
        println!("Please provide a file name");
        std::process::exit(1);
    }

    let calculation = arguments[1].clone();

}

impl Parser {
    fn new(&self, _input: &str) -> Ast {
        let mut parser = Parser {
            pos: 0,
            input: _input.to_string(),
        };

        self.parse_expr()
    }

    fn parse_expr(&self) -> Ast{
    }

}
