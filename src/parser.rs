#[derive(Debug)]
pub struct Parser {
    pos: usize,
    input: String,
}

#[derive(Debug)]
pub struct Ast {
    pub lhs: i32,
    pub rhs: i32,
    pub op: char,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser { pos: 0, input }
    }

    pub fn parse(&mut self) -> Ast {
        Ast {
            lhs: self
                .consume_while(|c| c.is_numeric())
                .parse::<i32>()
                .unwrap(),
            op: self.consume_char(),
            rhs: self
                .consume_while(|c| c.is_numeric())
                .parse::<i32>()
                .unwrap(),
        }
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();

        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }

        result
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, current_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));

        self.pos += next_pos;

        current_char
    }
}
