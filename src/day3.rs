use std::cell::RefCell;

#[derive(Debug, Copy, Clone)]
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

        self.Data.borrow_mut()[i * j] = s;
    }
}

pub fn part1() {
    let new_fab = Fabriq::new(10, 10);
    new_fab.set(0, 0, State::Used);
    println!("{:?}", new_fab);
}
