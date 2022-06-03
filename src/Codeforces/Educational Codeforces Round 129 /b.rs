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
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>,out: &mut W){
    let t = scan.token::<i32>();
    for _ in 0..t {
        let n = scan.token::<i32>();
        let mut v = Vec::with_capacity(n as usize);
        for _ in 0.. n {
            let temp = scan.token::<i32>();
            v.push(temp);
        }
        let mut sum = 0;
        let m = scan.token::<i32>();
        for _ in 0..m {
            let temp = scan.token::<i32>();
            sum += temp;
            sum = sum % n;
        }
        writeln!(out,"{}",v[sum as usize]).ok();
    }
}
fn main() {
    let (stdin,stdout) = (io::stdin(),io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan,&mut out);
}
