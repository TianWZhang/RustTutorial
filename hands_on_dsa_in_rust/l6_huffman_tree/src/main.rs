use std::{collections::BTreeMap, fs::File, io::Write};

#[derive(Debug)]
pub enum HuffNode {
    Tree(Box<HuffNode>, Box<HuffNode>),
    Leaf(char)
}

impl HuffNode {
    pub fn print_lfirst(&self, depth: i32, dir: char) {
        match self {
            Self::Tree(l, r) => {
                l.print_lfirst(depth + 1, '/');
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                println!("{}{}*", spc, dir);
                r.print_lfirst(depth+1, '\\');
            }
            Self::Leaf(c) => {
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                println!("{}{}{}",spc, dir, c);
            }
        }
    }

    pub fn encode_char(&self, c: char) -> Option<Vec<char>> {
        match self {
            Self::Tree(l, r) => {
                if let Some(mut v) = l.encode_char(c) {
                    v.insert(0, '0');
                    return Some(v);
                }
                if let Some(mut v) = r.encode_char(c) {
                    v.insert(0, '1');
                    return Some(v);
                }
                None
            }
            Self::Leaf(nc) => {
                if c == *nc {
                    return Some(Vec::new());
                } else {
                    None
                }
            }
        }
    }

    pub fn decode_char(&self, encoding: &[char], start: &mut usize) -> Option<char> {
        match self {
            Self::Tree(l, r) => {
                if encoding.len() <= *start {
                    return None;
                }
                if encoding[*start] == '0' {
                    *start += 1;
                    l.decode_char(encoding, start)
                } else {
                    *start += 1;
                    r.decode_char(encoding, start)
                }
            }
            Self::Leaf(nc) => {
                Some(*nc)
            }
        }
    }

    pub fn encode_str(&self, s: &str) -> Option<Vec<char>> {
        let mut res = Vec::new();
        for c in s.chars() {
            let v = self.encode_char(c)?;
            res.extend(v.into_iter());
        }
        Some(res)
    }

    pub fn compress(&self, s: &str, filename: &str) -> std::io::Result<()> {
        let mut filename = filename.to_owned();
        filename.push_str(".huf");
        let mut file = File::create(filename)?;
        file.write_all(self.encode_str(s).unwrap().into_iter().collect::<String>().as_bytes())?;
        Ok(())
    }

    pub fn decode_str(&self, encoding: &[char]) -> String {
        let mut res = Vec::new();
        let mut start = 0;
        while start < encoding.len() {
            if let Some(v) = self.decode_char(encoding, &mut start) {
                res.push(v);
            }
        }
        res.into_iter().collect()
    }
}

pub struct HScore {
    h: HuffNode,
    s: i32
}

pub fn build_tree(s: &str) -> HuffNode {
    let mut map = BTreeMap::new();
    for c in s.chars() {
        let n = *map.get(&c).unwrap_or(&0);
        map.insert(c, n+1);
    }
    let mut tlist: Vec<HScore> = map.into_iter().map(|(k, s)| HScore {
        h: HuffNode::Leaf(k),
        s
    }).collect();

    while tlist.len() > 1 {
        let last = tlist.len() - 1;
        for i in 0..last-1 {
            if tlist[i].s < tlist[last-1].s {
                tlist.swap(i, last-1);
            }
            if tlist[last-1].s < tlist[last].s {
                tlist.swap(last-1, last);
            }
        }
        let a_node = tlist.pop().unwrap();
        let b_node = tlist.pop().unwrap();
        let nnode = HuffNode::Tree(Box::new(a_node.h), Box::new(b_node.h));
        tlist.push(HScore {
            h: nnode,
            s: a_node.s + b_node.s
        });
    }
    tlist.pop().unwrap().h
}

fn main() {
    let s = "at an apple app";
    println!("{}", s);
    let t = build_tree(s);
    t.print_lfirst(0, '<');

    let n_encoding = t.encode_char('n').unwrap();
    println!("encoding of n = {:?}", n_encoding);
    let str_encoding = t.encode_str(s).unwrap();
    println!("encoding of str = {:?}", str_encoding);

    println!("decoding of n_encoding: {}", t.decode_char(&n_encoding, &mut 0).unwrap());
    println!("decoding of str_encoding: {}", t.decode_str(&str_encoding));
    t.compress(s, "test").unwrap();
}
