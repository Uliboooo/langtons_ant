use std::{env, fmt};

enum LR {
    Right,
    Left,
}
impl fmt::Display for LR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LR::Right => write!(f, "right"),
            LR::Left => write!(f, "left"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum BlackOrWhite {
    Black,
    White,
}

#[derive(Debug, PartialEq)]
struct Block {
    status: BlackOrWhite,
}
impl Default for Block {
    fn default() -> Self {
        Block {
            status: BlackOrWhite::White,
        }
    }
}

struct CurrentPoint {
    x: usize,
    y: usize,
}
impl CurrentPoint {
    // TODO: asの使えるトレイトをいつか
    fn new(x: i32, y: i32) -> Self {
        CurrentPoint {
            x: x as usize,
            y: y as usize,
        }
    }
    // fn up(place: &Place) -> Self {

    //     CurrentPoint { x: CurrentPoint{ x: place., y: todo!() } = , y: () }
    // }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turn_right(&mut self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
    fn turn_left(&mut self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

// TODO: Placeのメソッドで一括で場所指定して反転できる機能を実装
// TODO: remove unwrap
struct Place {
    place: Vec<Vec<Block>>,
    current_point: CurrentPoint,
    current_direction: Direction,
}
impl Place {
    fn new(len: i32, ini_point: CurrentPoint) -> Self {
        let mut place = Vec::<Vec<Block>>::new();
        // let mut line = Vec::<Block>::new();
        for _ in 0..len {
            let mut line = Vec::<Block>::new();
            for _ in 0..len {
                line.push(Block::default());
            }
            place.push(line);
        }
        Place {
            place,
            current_point: ini_point,
            current_direction: Direction::Up,
        }
    }
    fn len(&self) -> i32 {
        let mut count = 0;
        for _ in &self.place {
            count += 1;
        }
        count
    }
    fn show(&self) {
        for i in &self.place {
            for j in i {
                let res = if j.status == BlackOrWhite::Black {
                    "■"
                } else {
                    "□"
                };
                print!("{} ", res);
            }
            println!();
        }
    }
    fn back_is_black(&self) -> bool {
        self.place
            // 縦列
            .get(self.current_point.y)
            .unwrap()
            // 横列
            .get(self.current_point.x)
            .unwrap()
            .status
            == BlackOrWhite::Black
    }
    /// 一回の動作規則
    /// 方向の変更と進行
    fn action(&mut self, lr: LR) {
        match lr {
            LR::Right => self.current_direction = self.current_direction.turn_right(),
            LR::Left => self.current_direction = self.current_direction.turn_left(),
        }
        self.go();
        self.show();
    }
    // current_directionに1進める
    fn go(&mut self) {
        match self.current_direction {
            Direction::Up => {
                self.current_point =
                    CurrentPoint::new(self.current_point.x as i32, self.current_point.y as i32 - 1)
            }
            Direction::Down => {
                self.current_point =
                    CurrentPoint::new(self.current_point.x as i32, self.current_point.y as i32 + 1)
            }
            Direction::Left => {
                self.current_point =
                    CurrentPoint::new(self.current_point.x as i32 - 1, self.current_point.y as i32)
            }
            Direction::Right => {
                self.current_point =
                    CurrentPoint::new(self.current_point.x as i32 + 1, self.current_point.y as i32)
            }
        }
    }
}

#[test]
fn foo() {
    let f = Place::new(30, CurrentPoint { x: 5, y: 10 });
    f.show();
}

#[test]
fn place_test() {
    let f = Place::new(5, CurrentPoint::new(1, 1));
    assert_eq!(f.len(), 5);
}

// TODO: 現在の位置を表すもの
fn main() {
    dotenv::dotenv().ok();
    let loop_count = env::var("loop_count").unwrap().parse::<i32>().unwrap();

    let len = 51;
    let mut space = Place::new(len, CurrentPoint::new(len / 2, len / 2));
    for i in 0..loop_count {
        if space.back_is_black() {
            space.action(LR::Left);
        } else {
            space.action(LR::Right);
        }
        if i % 2 == 0 {
            space.show();
            println!();
        }
    }
}
