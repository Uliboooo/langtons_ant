use std::{env, fmt};
use serde::{Serialize, Deserialize};
// use serde_json;

mod export;

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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum BlackOrWhite {
    Black,
    White,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
impl Block {
    fn invert(&mut self) {
        match self.status {
            BlackOrWhite::Black => self.status = BlackOrWhite::White,
            BlackOrWhite::White => self.status = BlackOrWhite::Black,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Debug, Serialize, Deserialize)]
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

// TODO: remove unwrap
#[derive(Debug, Serialize, Deserialize)]
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
            current_direction: Direction::Left,
        }
    }
    /// Returns the length of this [`Place`].
    #[allow(dead_code)] //👈テスト用のため
    fn len(&self) -> i32 {
        let mut count = 0;
        for _ in &self.place {
            count += 1;
        }
        count
    }

    /// Returns the show of this [`Place`].
    fn show(&self) {
        //TODO: いつか一回のprintln!()で
        let mut result = String::new();
        for i in &self.place {
            let mut line_result = String::new();
            for j in i {
                line_result.push_str(if j.status == BlackOrWhite::Black {
                    "■"
                } else {
                    "□"
                });
                line_result.push(' ');
            }
            result.push_str(&format!("{}\n", line_result));
            // print!("{} ", line_result);
        }
        println!("{}\n", result);
    }
    /// current_pointの背景が黒ならtrue、白ならfalse
    fn back_is_black(&self) -> bool {
        self.get_current_status() == &BlackOrWhite::Black
    }
    /// 一回の動作.
    /// 方向の変更と進行.
    fn action(&mut self, lr: LR) {
        match lr {
            LR::Right => self.current_direction = self.current_direction.turn_right(),
            LR::Left => self.current_direction = self.current_direction.turn_left(),
        }
    }
    /// Returns the go of this [`Place`].
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
    fn invert(&mut self) {
        self.place
            .get_mut(self.current_point.y)
            .unwrap()
            .get_mut(self.current_point.x)
            .unwrap()
            .invert();
    }
    fn get_status(&self, x: usize, y: usize) -> &BlackOrWhite {
        &self.place.get(y).unwrap().get(x).unwrap().status
    }
    fn get_current_status(&self) -> &BlackOrWhite {
        self.get_status(self.current_point.x, self.current_point.y)
    }
}

#[test]
fn show_test() {
    let f = Place::new(51, CurrentPoint { x: 5, y: 10 });
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
    let len = env::var("len").unwrap().parse::<i32>().unwrap();

    let mut space = Place::new(len, CurrentPoint::new(len / 2, len / 2));
    for i in 1..=loop_count {
        if space.back_is_black() {
            // is black
            space.action(LR::Left);
        } else {
            // is white
            space.action(LR::Right);
        }
        space.invert();
        space.go();
        space.show();
        export::export(&space);
        println!("👆{}回目", i);
    }
    let mut foo = String::new();
    let _ = std::io::stdin().read_line(&mut foo).unwrap();
}
