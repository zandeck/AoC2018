use nom::*;
use std::cell::RefCell;
use std::str;
use std::str::FromStr;
use super::common;
use std::result::Result;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    NotUsed,
    Claimed(Vec<u32>),
}

#[derive(Debug)]
struct Fabriq {
    Height: usize,
    Width: usize,
    Data: Vec<State>,
}

#[derive(Debug, Clone, Copy)]
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
            Data: vec![State::NotUsed; Height * Width],
        }
    }

    fn set(&mut self, i: usize, j: usize, s: State) {
        assert!(i <= self.Height);
        assert!(j <= self.Width);
        //println!("Set Index ({:?}, {:?}): {:?}: ", i, j, i* self.Width + j);
        self.Data[ i* self.Width + j ] = s;
    }

    fn get(&self, i: usize, j:usize) -> State {
        assert!(i <= self.Height);
        assert!(j <= self.Width);
        //println!("Get Index ({:?}, {:?}): {:?}: ", i, j, i* self.Width + j);

        self.Data[i  * self.Width + j].clone()
    }

    fn claim(&mut self, IDClaim: u32, i: usize, j:usize) {
        let current_state = self.get(i, j);

        match current_state {
            State::NotUsed => self.set(i, j, State::Claimed( vec! [IDClaim] )),
            State::Claimed(d) => {    
                let mut claims = d.clone();
                claims.push(IDClaim);
                let new_state = State::Claimed( claims );
                self.set(i, j, new_state)

                }
        }
    }

    fn handle_claim(&mut self, c: Claim) {
        //println!("Handle a claim!!!");
        for i in c.from_top..(c.from_top + c.tall) {
            for j in c.from_left..(c.from_left + c.wide) {
                //println!("Claimed: {:?}, {:?}", i, j);
                self.claim(c.ID, i as usize, j as usize);
            }
        }
    }

    fn count_overlaped(&self) -> usize {
        let mut counter = 0;
        for i in 0..(self.Height) {
            for j in 0..(self.Width) {
                match self.get(i, j) {
                    State::Claimed(_) => counter = counter + 1,
                    _ => (),
                }
            }
        }
        counter
    }

    fn overlaped_claims(&self) -> HashSet<u32> {
        let mut ids = HashSet::new();
        for i in 0..(self.Height) {
            for j in 0..(self.Width) {
                match self.get(i, j) {
                    State::Claimed(d) => if d.len() >= 2 {d.into_iter().for_each(|e| { ids.insert(e); })},
                    _ => (),
                }
            }
        }

        ids
    }
}

pub fn part1() {
    //let mut new_fab = Fabriq::new(10, 10);
    //new_fab.set(0, 0, State::Claimed);
    // println!("{:?}", new_fab);
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
    let mut fabriq = Fabriq::new(1000, 1000);
    let mut claims_id: HashSet<u32> = HashSet::new();
    // println!("Claims: {:?}", claims);
    match claims {
        Ok(claims) => {
            for c in claims.1 {
                println!("Claim: {:?}", c);
                fabriq.handle_claim(c);
                claims_id.insert(c.ID);
            } 
        },
        _ => (),
    }
    let all_overlaping_claims: HashSet<u32> = fabriq.overlaped_claims();


    println!("Fabriq: {:?}", fabriq);
    println!("Overlaped: {:?}", fabriq.count_overlaped());
    println!("Overlaped claim: {:?}", claims_id.difference(&all_overlaping_claims).collect::<HashSet<&u32>>());

}