use std::fmt;
use std::io;

fn empty_screen() {
    for _ in 0..30 {
        print!("\n");
    }
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Clone, Copy)]
enum Value {
    None,
    O,
    X,
}

impl From<char> for Value {
    fn from(c: char) -> Self {
        match c {
            ' ' => Value::None,
            'O' => Value::O,
            'X' => Value::X,
            _ => Value::None,
        }
    }
}
impl From<Value> for char {
    fn from(v: Value) -> Self {
        char::from(&v)
    }
}
impl From<&Value> for char {
    fn from(v: &Value) -> Self {
        match v {
            Value::None => ' ',
            Value::O => 'O',
            Value::X => 'X',
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c: char = char::from(self);
        write!(f, "{}", c)
    }
}

struct Table {
    buf: [Value; 9],
}

impl Table {
    const fn new() -> Table {
        Table {
            buf: [
                Value::None,
                Value::X,
                Value::None,
                Value::None,
                Value::None,
                Value::None,
                Value::O,
                Value::None,
                Value::None,
            ],
        }
    }

    fn set(&mut self, x: usize, y: usize, v: Value) {
        assert!((0..3).contains(&x) && (0..3).contains(&y));
        self.buf[x * 3 + y] = v;
    }

    fn print(&self) {
        for row in 0..3 {
            for col in 0..3 {
                print!("{}", self.buf[row * 3 + col]);
                if col < 2 {
                    print!(" | ");
                } else {
                    print!("\n");
                }
            }
            if row < 2 {
                println!("----------");
            }
        }
    }
}
fn main() {
    let mut table = Table::new();
    table.print();

    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("fucked");

    let c: char =  c.trim().parse().expect("char");

    table.set(1,1,Value::from(c));

    empty_screen();
    table.print();
}
