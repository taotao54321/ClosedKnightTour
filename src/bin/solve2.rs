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

    macro_rules! ret {
        () => {{
            board.remove(xy);
            return;
        }};
    }

    if xy == XY_LAST {
        if board.is_completed() {
            //eprintln!("{}", board);
            *ans += 1;
            //if *ans % 100 == 0 {
            //    eprint!(".");
            //}
        }
        ret!();
    }

    // 枝刈り
    //
    // 行き先候補のうち、隣接空きマス数が 0 の頂点があれば(v とする)それ以外は調べなくてよい。v 以
    // 外の頂点に行った場合、もう v には行けなくなるため。
    //
    // 行き先候補のうち、「ゴールでなく、かつ隣接空きマス数が 1 の頂点」(*) があればそれ以外は調べ
    // なくてよい。v 以外の頂点に行ってから v に行った場合、そこで行き止まりになるため。
    // このことから、条件 (*) を満たす候補が複数あれば直ちに枝刈りできることもわかる。

    let mut n_deg1 = 0;
    let mut to_deg1 = None;
    for to in neighbors(xy) {
        if !board.is_empty(to) {
            continue;
        }
        match board.get_deg(to) {
            0 => {
                rec(ans, board, to);
                ret!();
            }
            1 if to != XY_LAST => {
                n_deg1 += 1;
                to_deg1 = Some(to);
            }
            _ => {}
        }
    }
    match n_deg1 {
        0 => {}
        1 => {
            rec(ans, board, to_deg1.unwrap());
            ret!();
        }
        _ => {
            ret!();
        }
    }

    for to in neighbors(xy) {
        if !board.is_empty(to) {
            continue;
        }
        rec(ans, board, to);
    }

    ret!();
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
    degs: Vec<u32>, // 隣接する空きマスの個数
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

        let degs: Vec<_> = itertools::iproduct!(0..h, 0..w)
            .map(|(y, x)| {
                if (2..w - 2).contains(&x) && (2..h - 2).contains(&y) {
                    neighbors((x, y))
                        .filter(|(x, y)| cells[(w * y + x) as usize] == 0)
                        .count() as u32
                } else {
                    0
                }
            })
            .collect();

        Self {
            w,
            h,
            n: 0,
            cells,
            degs,
        }
    }

    fn n_remain(&self) -> u32 {
        self.n - self.w_inner() * self.h_inner()
    }

    fn w_inner(&self) -> u32 {
        self.w - 4
    }

    fn h_inner(&self) -> u32 {
        self.h - 4
    }

    fn is_completed(&self) -> bool {
        self.n_remain() == 0
    }

    fn get(&self, xy: (u32, u32)) -> u32 {
        self.cells[self.xy2idx(xy)]
    }

    fn is_empty(&self, (x, y): (u32, u32)) -> bool {
        self.get((x, y)) == 0
    }

    fn get_deg(&self, xy: (u32, u32)) -> u32 {
        self.degs[self.xy2idx(xy)]
    }

    fn put(&mut self, (x, y): (u32, u32)) {
        debug_assert!(self.is_empty((x, y)));
        let idx = self.xy2idx((x, y));
        self.n += 1;
        self.cells[idx] = self.n;

        for to in neighbors((x, y)) {
            if !self.is_empty(to) {
                continue;
            }
            let idx = self.xy2idx(to);
            self.degs[idx] -= 1;
        }
    }

    fn remove(&mut self, (x, y): (u32, u32)) {
        debug_assert_ne!(self.n, 0);
        debug_assert!(!self.is_empty((x, y)));
        let idx = self.xy2idx((x, y));
        self.n -= 1;
        self.cells[idx] = 0;

        for to in neighbors((x, y)) {
            if !self.is_empty(to) {
                continue;
            }
            let idx = self.xy2idx(to);
            self.degs[idx] += 1;
        }
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

        writeln!(f)?;

        for y in 2..self.h - 2 {
            for x in 2..self.w - 2 {
                write!(f, "{} ", self.get_deg((x, y)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
