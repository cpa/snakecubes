struct Cube {
    cubelets: [Cubelet; 27],
}

struct Cubelet {
    outface: Face
}

enum Face {
    Through, Turn(Rotation)
}

enum Rotation {
    R1Pos, R1Neg, R2Pos, R2Neg
}

#[derive(Clone)]
enum Direction {
    XPos, XNeg, YPos, YNeg, ZPos, ZNeg
}

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

fn coordinates(cube: &Cube) -> [(i8, i8, i8); 27] {
    let (mut cur_x, mut cur_y, mut cur_z) = (0, 0, 0);
    let mut cur_direction = Direction::XPos;
    let mut coords = [(0, 0, 0); 27];

    for (idx, cubelet) in cube.cubelets.iter().enumerate() {
        coords[idx] = (cur_x, cur_y, cur_z);

        // compute outgoing direction for this cubelet
        cur_direction = match &cubelet.outface {
            Face::Through => cur_direction,
            Face::Turn(rot) => turned(cur_direction, rot),
        };

        // step to next cubelet along outgoing direction
        step(&mut cur_x, &mut cur_y, &mut cur_z, &cur_direction);
    }

    coords
}


fn is_solved(coords: [(i8, i8, i8); 27]) -> bool {
    let (min, max) = coords[1..].iter().fold((coords[0], coords[0]), |(min, max), &p| {
        (
            (min.0.min(p.0), min.1.min(p.1), min.2.min(p.2)),
            (max.0.max(p.0), max.1.max(p.1), max.2.max(p.2)),
        )
    });

    if (max.0 - min.0) != 2 || (max.1 - min.1) != 2 || (max.2 - min.2) != 2 {
        return false;
    }

    for i in 0..27 {
        for j in (i + 1)..27 {
            if coords[i] == coords[j] {
                return false;
            }
        }
    }

    true
}


fn solve(cube: &mut Cube, n: usize) -> () {

    // println!("{}", n);

    if is_solved(coordinates(&cube)) {
        println!("{:?}", coordinates(&cube));
        return ();
    }

    if n == 27 {
        return ();
    }

    match &cube.cubelets[n].outface {
        Face::Through => solve(cube, n+1),
        Face::Turn(_rotation) => {
            cube.cubelets[n].outface = Face::Turn(Rotation::R1Neg);
            solve(cube, n+1);
            cube.cubelets[n].outface = Face::Turn(Rotation::R1Pos);
            solve(cube, n+1);
            cube.cubelets[n].outface = Face::Turn(Rotation::R2Neg);
            solve(cube, n+1);
            cube.cubelets[n].outface = Face::Turn(Rotation::R2Pos);
            solve(cube, n+1);
        }
    }
}

fn main() {

    let mut cube = Cube {
        cubelets: [
            Cubelet { outface: Face::Through }, Cubelet { outface: Face::Through }, 
            Cubelet { outface: Face::Turn(Rotation::R1Pos) }, Cubelet { outface: Face::Through }, 
            Cubelet { outface: Face::Turn(Rotation::R1Pos) }, Cubelet { outface: Face::Through }, 
            Cubelet { outface: Face::Turn(Rotation::R1Pos) }, Cubelet { outface: Face::Through }, 
            Cubelet { outface: Face::Turn(Rotation::R1Pos) }, Cubelet { outface: Face::Turn(Rotation::R1Pos) }, 
            Cubelet { outface: Face::Turn(Rotation::R1Pos) }, Cubelet { outface: Face::Turn(Rotation::R1Pos) }, 
            Cubelet { outface: Face::Through }, Cubelet { outface: Face::Turn(Rotation::R1Pos) }, 
            Cubelet { outface: Face::Through }, Cubelet { outface: Face::Turn(Rotation::R1Pos) }, 
            Cubelet { outface: Face::Turn(Rotation::R1Pos) }, Cubelet { outface: Face::Turn(Rotation::R1Pos) }, 
            Cubelet { outface: Face::Through }, Cubelet { outface: Face::Turn(Rotation::R1Pos) }, 
            Cubelet { outface: Face::Turn(Rotation::R1Pos) }, Cubelet { outface: Face::Through }, 
            Cubelet { outface: Face::Turn(Rotation::R1Pos) }, Cubelet { outface: Face::Turn(Rotation::R1Pos) }, 
            Cubelet { outface: Face::Turn(Rotation::R1Pos) }, Cubelet { outface: Face::Through }, 
            Cubelet { outface: Face::Through }]
    };

    println!("{:?}", coordinates(&cube));
    println!("{}", is_solved(coordinates(&cube)));

    solve(&mut cube, 0);
}
