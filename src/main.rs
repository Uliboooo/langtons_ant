use core::fmt;

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

struct LinePlace {
    line: Vec<Block>,
    // len: u8,
}
impl LinePlace {
    fn new(len: i32) -> Self {
        let mut line = Vec::<Block>::new();
        for _ in 0..len {
            line.push(Block::default());
        }
        LinePlace { line }
    }
}

// TODO: Placeのメソッドで一括で場所指定して反転できる機能を実装
// TODO: remove unwrap
struct Place {
    place: Vec<Vec<Block>>,
    current_point: (i32, i32),
}
impl Place {
    fn new(len: i32, ini_point: (i32, i32)) -> Self {
        let mut place = Vec::<Vec<Block>>::new();
        // let mut line = Vec::<Block>::new();
        for _ in 0..len {
            let mut line = Vec::<Block>::new();
            for _ in 0..len {
                line.push(Block::default());
            }
            place.push(line);
        }
        Place { place, current_point: ini_point }
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
                let res= if j.status == BlackOrWhite::Black {
                    "■"
                } else {
                    "□"
                };
                print!("{} ", res);
            }
            println!();
        }
    }
    fn block_status(&self, point: (usize, usize)) -> BlackOrWhite {
        // let a = self.place.get(0).unwrap().get(0).unwrap();
        if self.place.get(point.0).unwrap().get(point.1).unwrap().status == BlackOrWhite::Black {
            BlackOrWhite::Black
        } else {
            BlackOrWhite::White
        }
    }
    // fn get(&self, point: (i32, i32)) -> Block {
    //     // let a = self.place;
    //     let t = self.place.get(0).unwrap();
    // }
}

#[test]
fn foo() {
    let f = Place::new(100, (3, 4));
    f.show();
}

#[test]
fn place_test() {
    let f = Place::new(5, (1, 1));
    assert_eq!(f.len(), 5);
}

// struct Status {
//     current_point : (i32, i32),
//     current_place: Place,
// }
// impl Status {
//     fn new(len: i32) -> Self{
//         Status { current_point: (0, 0), current_place: Place::new(len, (0, 0)) }
//     }
//     // fn set(mut self, len: i32, point: (i32, i32)) -> Self {
//     //     self.current_place = Place::new(len);
//     //     self.current_point = point;
//     //     self
//     // }
// }

fn main() {
    let len = 200;
    let space = Place::new(len, (len / 2, len / 2));
    println!("Hello, world!");
}
