use std::cmp;
use std::io;
use std::str;

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed reader");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let t = scan.token::<i32>();
    for _ in 0..t {
        let n = scan.token::<i32>();
        let mut alice = -1;
        for _ in 0..n {
            let a = scan.token::<i32>();
            alice = cmp::max(a, alice);
        }
        let m = scan.token::<i32>();
        let mut bob = -1;
        for _ in 0..m {
            let b = scan.token::<i32>();
            bob = cmp::max(b, bob);
        }
        let (result1, result2) = if alice > bob {
            (String::from("Alice"), String::from("Alice"))
        } else if alice == bob {
            (String::from("Alice"), String::from("Bob"))
        } else {
            (String::from("Bob"), String::from("Bob"))
        };
        writeln!(out, "{}", result1).ok();
        writeln!(out, "{}", result2).ok();
    }
}
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
