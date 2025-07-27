use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    L(Vec<Packet>),
    V(i32),
}

impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        fn parse_tokens(tokens: &mut std::iter::Peekable<std::str::Chars>) -> Packet {
            match tokens.next() {
                Some('[') => {
                    let mut items = Vec::new();
                    while tokens.peek() != Some(&']') {
                        items.push(parse_tokens(tokens));
                        if tokens.peek() == Some(&',') {
                            tokens.next(); // consume comma
                        }
                    }
                    tokens.next(); // consume ']'
                    Packet::L(items)
                }
                Some(c) if c.is_digit(10) => {
                    let mut num = String::from(c);
                    while let Some(&c) = tokens.peek() {
                        if c.is_digit(10) {
                            num.push(tokens.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    Packet::V(num.parse().unwrap())
                }
                _ => panic!("Invalid token"),
            }
        }
        
        parse_tokens(&mut s.chars().peekable())
    }
}

fn parse_packets(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|pair| {
            let (left, right) = pair.split_once('\n').unwrap();
            (Packet::from(left), Packet::from(right))
        })
        .collect()
}

fn task_1(left: &Packet, right: &Packet) -> bool {
    false
}


pub fn main(input: String) -> Result<(), Box<dyn Error>> {
    parse_packets(&input)
        .iter()
        .for_each(|packet| println!("{:?}", packet));
    Ok(())
}
