use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

struct Cube {
    cubelets: [Cubelet; 27],
}

struct Cubelet {
    outface: Face,
}

enum Face {
    Through,
    Turn(Rotation),
}

enum Rotation {
    R1Pos,
    R1Neg,
    R2Pos,
    R2Neg,
}

#[derive(Clone)]
enum Direction {
    XPos,
    XNeg,
    YPos,
    YNeg,
    ZPos,
    ZNeg,
}

#[inline]
fn step(x: &mut i8, y: &mut i8, z: &mut i8, dir: &Direction) {
    match dir {
        Direction::XPos => *x += 1,
        Direction::XNeg => *x -= 1,
        Direction::YPos => *y += 1,
        Direction::YNeg => *y -= 1,
        Direction::ZPos => *z += 1,
        Direction::ZNeg => *z -= 1,
    }
}

#[inline]
fn turned(cur: Direction, rot: &Rotation) -> Direction {
    match cur {
        Direction::XPos | Direction::XNeg => match rot {
            Rotation::R1Pos => Direction::YPos,
            Rotation::R1Neg => Direction::YNeg,
            Rotation::R2Pos => Direction::ZPos,
            Rotation::R2Neg => Direction::ZNeg,
        },
        Direction::YPos | Direction::YNeg => match rot {
            Rotation::R1Pos => Direction::ZPos,
            Rotation::R1Neg => Direction::ZNeg,
            Rotation::R2Pos => Direction::XPos,
            Rotation::R2Neg => Direction::XNeg,
        },
        Direction::ZPos | Direction::ZNeg => match rot {
            Rotation::R1Pos => Direction::XPos,
            Rotation::R1Neg => Direction::XNeg,
            Rotation::R2Pos => Direction::YPos,
            Rotation::R2Neg => Direction::YNeg,
        },
    }
}

#[inline]
fn coordinates(cube: &Cube) -> [(i8, i8, i8); 27] {
    let (mut cur_x, mut cur_y, mut cur_z) = (0, 0, 0);
    let mut cur_direction = Direction::XPos;
    let mut coords = [(0, 0, 0); 27];

    for (idx, cubelet) in cube.cubelets.iter().enumerate() {
        coords[idx] = (cur_x, cur_y, cur_z);

        cur_direction = match &cubelet.outface {
            Face::Through => cur_direction,
            Face::Turn(rot) => turned(cur_direction, rot),
        };

        step(&mut cur_x, &mut cur_y, &mut cur_z, &cur_direction);
    }

    coords
}

#[inline]
fn is_solved(coords: [(i8, i8, i8); 27]) -> bool {
    let (mut min_x, mut min_y, mut min_z) = (100, 100, 100);
    let (mut max_x, mut max_y, mut max_z) = (-100, -100, -100);
    for (x, y, z) in coords {
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
        if z < min_z {
            min_z = z;
        }
        if z > max_z {
            max_z = z;
        }
        if (max_x - min_x) > 2 || (max_y - min_y) > 2 || (max_z - min_z) > 2 {
            return false;
        }
    }

    let mut seen = HashSet::with_capacity(coords.len());

    (max_x - min_x) == 2
        && (max_y - min_y) == 2
        && (max_z - min_z) == 2
        && coords.iter().all(|x| seen.insert(x))

}

fn solve(cube: &mut Cube, n: usize, counter: &AtomicUsize) {
    counter.fetch_add(1, Ordering::Relaxed);

    if is_solved(coordinates(cube)) {
        println!("{:?}", coordinates(cube));
        return;
    }

    if n == 27 {
        return;
    }

    match &cube.cubelets[n].outface {
        Face::Through => solve(cube, n + 1, counter),
        Face::Turn(_rotation) => {
            cube.cubelets[n].outface = Face::Turn(Rotation::R1Neg);
            solve(cube, n + 1, counter);
            cube.cubelets[n].outface = Face::Turn(Rotation::R1Pos);
            solve(cube, n + 1, counter);
            cube.cubelets[n].outface = Face::Turn(Rotation::R2Neg);
            solve(cube, n + 1, counter);
            cube.cubelets[n].outface = Face::Turn(Rotation::R2Pos);
            solve(cube, n + 1, counter);
        }
    }
}

fn main() {
    let counter = AtomicUsize::new(0);
    let stop = AtomicBool::new(false);

    // Our starting cubestring, as can be seen in pic1.jpg
    let mut cube = Cube {
        cubelets: [
            Cubelet {
                outface: Face::Through,
            },
            Cubelet {
                outface: Face::Through,
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Through,
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Through,
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Through,
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Through,
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Through,
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Through,
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Through,
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Turn(Rotation::R1Pos),
            },
            Cubelet {
                outface: Face::Through,
            },
            Cubelet {
                outface: Face::Through,
            },
        ],
    };

    thread::scope(|s| {
        s.spawn(|| {
            while !stop.load(Ordering::Relaxed) {
                println!("Counter: {}", counter.load(Ordering::Relaxed));
                thread::sleep(Duration::from_secs(3));
            }
        });
        solve(&mut cube, 0, &counter);
        stop.store(true, Ordering::Relaxed);
    });

}
