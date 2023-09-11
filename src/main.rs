#[allow(unused_imports)]
use std::cmp::{min,max};
#[allow(unused_imports)]
use std::{io::{BufWriter, stdin, stdout, Write}, char::ToUppercase};
 
struct Scanner<R> {
    reader: R,
    line: Vec<u8>,
    ptr: usize
 
}
#[allow(dead_code)]
impl<R: std::io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self{reader, line: vec![], ptr: 0}
    }
 
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            while self.ptr < self.line.len() && self.line[self.ptr].is_ascii_whitespace() {
                self.ptr += 1;
            }
            if self.ptr != self.line.len() {
                let start = self.ptr;
                while self.ptr < self.line.len() && !self.line[self.ptr].is_ascii_whitespace() {
                    self.ptr += 1;
                }
                return std::str::from_utf8(&self.line[start..self.ptr]).unwrap().parse().ok().
                    expect("parse error");
            }
            self.line.clear();
            self.reader.read_until(b'\n', &mut self.line).expect("read error");
            self.ptr = 0;
        }
    }
 
    fn line(&mut self) -> Vec<u8> {
        if self.ptr == self.line.len() {
            self.line.clear();
            self.reader.read_until(b'\n', &mut self.line).expect("read error");
            self.ptr = 0;
        }
        let result = self.line[self.ptr..].to_vec();
        self.ptr = self.line.len();
        return result;
    }
 
    fn eof(&mut self) -> bool {
        loop {
            while self.ptr < self.line.len() && self.line[self.ptr].is_ascii_whitespace() {
                self.ptr += 1;
            }
            if self.ptr != self.line.len() {
                return false;
            }
            self.line.clear();
            self.ptr = 0;
            if let Ok(0) = self.reader.read_until(b'\n', &mut self.line) {
                return true;
            }
        }
    }
 
}

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let mut out = BufWriter::new(stdout.lock());
    let mut scan = Scanner::new(stdin.lock());
    // let t = scan.next::<isize>();
    // for _ in 0..t {
        let n = scan.next::<i64>();
        let nn = n as usize;
        let m = scan.next::<i64>();
        let seed = 9991;
        let mut pre: [i64; 200005] = [0; 200005];
        let mut exist: [Vec<(i64, i64)>; 10000] = [(); 10000].map(|_| Vec::new()) ;
        let s: Vec<i64> = (0..n).map(|_| scan.next()).collect();
    //     let f: Vec<isize> = (0..n).map(|_| scan.next()).collect();
        pre[0] = s[0];
        for i in 1..nn {
            pre[i] = s[i] + pre[i - 1];
        }
        exist[0].push((0, 1));
        let mut ans:i64 = 0;
        for i in 0..nn {
            if pre[i] - m >= 0 {
                let kk = ((pre[i] - m) % seed ) as usize;
                for pa in &exist[kk]{
                    if pa.0 == pre[i] - m {
                        ans += pa.1;
                    }
                }
            }
            let kk = (pre[i] % seed) as usize;
            let mut flag = true;
            for pa in &mut exist[kk]{
                if pa.0 == pre[i] - m {
                    pa.1 += 1;
                    flag = false;
                }
            }
            if flag {
                exist[kk].push((pre[i], 1));
            }

        }
        writeln!(out, "{}", ans).unwrap();
    // }
}