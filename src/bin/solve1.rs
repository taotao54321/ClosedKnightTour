use structopt::StructOpt;

const XY_FIRST: (u32, u32) = (2, 2);
const XY_SECOND: (u32, u32) = (4, 3);
const XY_LAST: (u32, u32) = (3, 4);

#[derive(Debug, StructOpt)]
struct Opt {
    w: u32,
    h: u32,
}

fn main() {
    let opt = Opt::from_args();

    let mut board = Board::new(opt.w, opt.h);
    board.put(XY_FIRST);

    let mut ans = 0;
    rec(&mut ans, &mut board, XY_SECOND);

    println!("{}", ans);
}

fn rec(ans: &mut u64, board: &mut Board, xy: (u32, u32)) {
    board.put(xy);

    if xy == XY_LAST {
        if board.is_completed() {
            //eprintln!("{}", board);
            *ans += 1;
            //if *ans % 100 == 0 {
            //    eprint!(".");
            //}
        }
        board.remove(xy);
        return;
    }

    if board.n() < board.w_inner() * board.h_inner() - 1 {
        if neighbors(xy).any(|xy| board.is_deadend(xy)) {
            board.remove(xy);
            return;
        }
    }

    for xy in neighbors(xy) {
        if !board.is_empty(xy) {
            continue;
        }
        rec(ans, board, xy);
    }

    board.remove(xy);
}

fn neighbors((x, y): (u32, u32)) -> impl Iterator<Item = (u32, u32)> {
    const DXDYS: [(i32, i32); 8] = [
        (-1, -2),
        (1, -2),
        (-2, -1),
        (2, -1),
        (-2, 1),
        (2, 1),
        (-1, 2),
        (1, 2),
    ];

    DXDYS
        .iter()
        .map(move |(dx, dy)| ((x as i32 + dx) as u32, (y as i32 + dy) as u32))
}

#[derive(Debug)]
struct Board {
    w: u32,
    h: u32,
    n: u32,
    cells: Vec<u32>,
}

impl Board {
    fn new(w_inner: u32, h_inner: u32) -> Self {
        assert!(w_inner >= 3);
        assert!(h_inner >= 3);

        let w = w_inner + 4;
        let h = h_inner + 4;

        let cells: Vec<_> = itertools::iproduct!(0..h, 0..w)
            .map(|(y, x)| {
                if (2..w - 2).contains(&x) && (2..h - 2).contains(&y) {
                    0
                } else {
                    u32::max_value()
                }
            })
            .collect();

        Self { w, h, n: 0, cells }
    }

    fn w_inner(&self) -> u32 {
        self.w - 4
    }

    fn h_inner(&self) -> u32 {
        self.h - 4
    }

    fn n(&self) -> u32 {
        self.n
    }

    fn is_completed(&self) -> bool {
        self.n == self.w_inner() * self.h_inner()
    }

    fn get(&self, (x, y): (u32, u32)) -> u32 {
        self.cells[self.xy2idx((x, y))]
    }

    fn is_empty(&self, (x, y): (u32, u32)) -> bool {
        self.get((x, y)) == 0
    }

    fn is_deadend(&self, xy: (u32, u32)) -> bool {
        self.is_empty(xy) && neighbors(xy).all(|xy| !self.is_empty(xy))
    }

    fn put(&mut self, (x, y): (u32, u32)) {
        debug_assert!(self.is_empty((x, y)));
        let idx = self.xy2idx((x, y));
        self.n += 1;
        self.cells[idx] = self.n;
    }

    fn remove(&mut self, (x, y): (u32, u32)) {
        debug_assert_ne!(self.n, 0);
        debug_assert!(!self.is_empty((x, y)));
        let idx = self.xy2idx((x, y));
        self.n -= 1;
        self.cells[idx] = 0;
    }

    fn xy2idx(&self, (x, y): (u32, u32)) -> usize {
        (self.w * y + x) as usize
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 2..self.h - 2 {
            for x in 2..self.w - 2 {
                write!(f, "{:02} ", self.get((x, y)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
