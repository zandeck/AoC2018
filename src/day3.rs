use nom::*;
use std::cell::RefCell;
use std::str;
use std::str::FromStr;
use super::common;
use std::result::Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    NotUsed,
    Used,
    Overlaped,
}

#[derive(Debug)]
struct Fabriq {
    Height: usize,
    Width: usize,
    Data: RefCell<Vec<State>>,
}

#[derive(Debug)]
struct Claim {
    ID: u32,
    from_left: u32,
    from_top: u32,
    wide: u32,
    tall: u32,
}

impl Claim {
    fn new(ID: u32, from_left: u32, from_top: u32, wide: u32, tall: u32) -> Claim {
        Claim {
            ID,
            from_left,
            from_top,
            wide,
            tall,
        }
    }
}

impl Fabriq {
    fn new(Height: usize, Width: usize) -> Fabriq {
        Fabriq {
            Height,
            Width,
            Data: RefCell::new(vec![State::NotUsed; Height * Width]),
        }
    }

    fn set(&self, i: usize, j: usize, s: State) {
        assert!(i <= self.Height);
        assert!(j <= self.Width);
        println!("Set Index ({:?}, {:?}): {:?}: ", i, j, i* self.Width + j);
        self.Data.borrow_mut()[ i* self.Width + j ] = s;
    }

    fn get(&self, i: usize, j:usize) -> State {
        assert!(i <= self.Height);
        assert!(j <= self.Width);
        println!("Get Index ({:?}, {:?}): {:?}: ", i, j, i* self.Width + j);

        self.Data.borrow()[i  * self.Width + j]
    }

    fn claim(&self, i: usize, j:usize) {
        let current_state = self.get(i, j);
        if current_state == State::NotUsed {
            self.set(i, j, State::Used);
        }
        else if current_state == State::Used {
            self.set(i, j, State::Overlaped);
        }
    }

    fn handle_claim(&self, c: Claim) {
        println!("Handle a claim!!!");
        for i in c.from_top..(c.from_top + c.tall) {
            for j in c.from_left..(c.from_left + c.wide) {
                println!("Claimed: {:?}, {:?}", i, j);
                self.claim(i as usize, j as usize);
            }
        }
    }

    fn count_overlaped(&self) -> usize {
        let mut counter = 0;
        for i in 0..(self.Height) {
            for j in 0..(self.Width) {
                if self.get(i, j) == State::Overlaped {
                    counter = counter + 1;
                }
            }
        }
        counter
    }
}

pub fn part1() {
    let new_fab = Fabriq::new(10, 10);
    new_fab.set(0, 0, State::Used);
    println!("{:?}", new_fab);
}

fn u8_to_u32(l: nom::types::CompleteStr) -> u32 {
    u32::from_str(&l).unwrap()
}

named!(
    parse_claim< nom::types::CompleteStr, Claim >,
    do_parse!(
        tag!("#") >>
        claimID: ws!(digit) >>
        ws!(tag!("@")) >>
        from_left: ws!(digit) >>
        tag!(",") >>
        from_top: ws!(digit) >>
        ws!(tag!(":")) >> 
        wide: ws!(digit) >>
        ws!(tag!("x")) >>
        tall: ws!(digit) >>
        (   Claim::new(
            u8_to_u32(claimID),
            u8_to_u32(from_left),
            u8_to_u32(from_top),
            u8_to_u32(wide),
            u8_to_u32(tall) 
            )
        )
    )
);

named!(
    parse_claims< nom::types::CompleteStr, Vec<Claim> >,
    many0!(ws!(parse_claim))
);

pub fn part2() {
    let data = common::import_file_to_u8("resources/input3.txt".to_string()).unwrap();
    let claims = parse_claims(nom::types::CompleteStr(&data));
    let fabriq = Fabriq::new(1001, 1001);
    
    // println!("Claims: {:?}", claims);
    match claims {
        Ok(claims) => {
            for c in claims.1 {
                println!("Claim: {:?}", c);
                fabriq.handle_claim(c);
            } 
        },
        _ => (),
    }

    // println!("Fabriq: {:?}", fabriq);
    println!("Overlaped: {:?}", fabriq.count_overlaped());

}