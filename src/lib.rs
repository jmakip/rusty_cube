#[derive(Clone, Copy)]
pub struct Edge {
    id: u16,
    orient: i8,
}
#[derive(Clone, Copy)]
pub struct Corner {
    id: u16,
    orient: i8,
}
#[derive(Clone, Copy)]
pub struct Cube {
    edges: [Edge; 12],
    corners: [Corner; 8],
}

#[derive(Clone)]
pub struct SolveState
{
  pub  dist: u32,
    pub alg: Vec<CubeTurn>,
 pub   state: Cube,
   pub valid_turns: Vec<CubeTurn>,
}
#[derive(PartialEq, Clone, Copy)]
pub enum CubeTurn {
    Up,
    UpI,
    Up2,
    Right,
    RightI,
    Right2,
    Front,
    FrontI,
    Front2,
    Left,
    LeftI,
    Left2,
    Down,
    DownI,
    Down2,
    Back,
    BackI,
    Back2,
}

pub fn solved_cube() -> Cube {
    Cube {
        edges: [
            Edge { id: 1, orient: 0 },
            Edge { id: 2, orient: 0 },
            Edge { id: 3, orient: 0 },
            Edge { id: 4, orient: 0 },
            Edge { id: 5, orient: 0 },
            Edge { id: 6, orient: 0 },
            Edge { id: 7, orient: 0 },
            Edge { id: 8, orient: 0 },
            Edge { id: 9, orient: 0 },
            Edge { id: 10, orient: 0 },
            Edge { id: 11, orient: 0 },
            Edge { id: 12, orient: 0 },
        ],
        corners: [
            Corner { id: 1, orient: 0 },
            Corner { id: 2, orient: 0 },
            Corner { id: 3, orient: 0 },
            Corner { id: 4, orient: 0 },
            Corner { id: 5, orient: 0 },
            Corner { id: 6, orient: 0 },
            Corner { id: 7, orient: 0 },
            Corner { id: 8, orient: 0 },
        ],
    }
}
pub fn pos_as_string(cube: Cube) -> String {
    let mut pos = String::from("");
    for e in cube.edges.iter() {
        match e.orient {
            1 => pos.push_str("-"),
            0 => {}
            _ => pos.push_str("!"),
        }
        match e.id {
            1 => pos.push_str("UF"),
            2 => pos.push_str("UR"),
            3 => pos.push_str("UB"),
            4 => pos.push_str("UL"),

            5 => pos.push_str("DF"),
            6 => pos.push_str("DR"),
            7 => pos.push_str("DB"),
            8 => pos.push_str("DL"),

            9 => pos.push_str("FR"),
            10 => pos.push_str("FL"),
            11 => pos.push_str("BR"),
            12 => pos.push_str("BL"),
            _ => pos.push_str("?"),
        }
        pos.push_str(" ");
    }
    for e in cube.corners.iter() {
        match e.orient {
            1 => pos.push_str("+"),
            -1 => pos.push_str("-"),
            0 => {}
            _ => pos.push_str("!"),
        }
        match e.id {
            1 => pos.push_str("UFR"),
            2 => pos.push_str("URB"),
            3 => pos.push_str("UBL"),
            4 => pos.push_str("ULF"),

            5 => pos.push_str("DRF"),
            6 => pos.push_str("DFL"),
            7 => pos.push_str("DLB"),
            8 => pos.push_str("DBR"),
            _ => pos.push_str("?"),
        }
        pos.push_str(" ");
    }

    return pos;
}

pub fn print_cubies(cube: Cube) {
    let pos = pos_as_string(cube);
    println!("{}", pos);
}

fn corner_plus(mut corner: Corner) -> Corner {
    if corner.orient < 1 {
        corner.orient += 1;
    } else {
        corner.orient = -1;
    }
    return corner;
}

fn corner_minus(mut corner: Corner) -> Corner {
    if corner.orient > -1 {
        corner.orient -= 1;
    } else {
        corner.orient = 1;
    }
    return corner;
}
fn edge_flip(mut edge: Edge) -> Edge {
    if edge.orient == 0 {
        edge.orient = 1;
    } else {
        edge.orient = 0;
    }
    return edge;
}

pub fn rotate_u(cube: Cube) -> Cube {
    let mut cube_trans = cube.clone();
    cube_trans.edges[0] = cube.edges[1];
    cube_trans.edges[1] = cube.edges[2];
    cube_trans.edges[2] = cube.edges[3];
    cube_trans.edges[3] = cube.edges[0];

    cube_trans.corners[0] = cube.corners[1];
    cube_trans.corners[1] = cube.corners[2];
    cube_trans.corners[2] = cube.corners[3];
    cube_trans.corners[3] = cube.corners[0];
    return cube_trans;
}

pub fn rotate_ui(cube: Cube) -> Cube {
    let mut cube_trans = cube.clone();
    cube_trans.edges[0] = cube.edges[3];
    cube_trans.edges[1] = cube.edges[0];
    cube_trans.edges[2] = cube.edges[1];
    cube_trans.edges[3] = cube.edges[2];

    cube_trans.corners[0] = cube.corners[3];
    cube_trans.corners[1] = cube.corners[0];
    cube_trans.corners[2] = cube.corners[1];
    cube_trans.corners[3] = cube.corners[2];
    return cube_trans;
}

pub fn rotate_r(cube: Cube) -> Cube {
    let mut cube_trans = cube.clone();
    cube_trans.edges[1] = cube.edges[8];
    cube_trans.edges[8] = cube.edges[5];
    cube_trans.edges[5] = cube.edges[10];
    cube_trans.edges[10] = cube.edges[1];

    cube_trans.corners[0] = corner_minus(cube.corners[4]);
    cube_trans.corners[1] = corner_plus(cube.corners[0]);
    cube_trans.corners[7] = corner_minus(cube.corners[1]);
    cube_trans.corners[4] = corner_plus(cube.corners[7]);
    return cube_trans;
}

pub fn rotate_ri(cube: Cube) -> Cube {
    let mut cube_trans = cube.clone();
    cube_trans.edges[8] = cube.edges[1];
    cube_trans.edges[5] = cube.edges[8];
    cube_trans.edges[10] = cube.edges[5];
    cube_trans.edges[1] = cube.edges[10];

    cube_trans.corners[4] = corner_plus(cube.corners[0]);
    cube_trans.corners[0] = corner_minus(cube.corners[1]);
    cube_trans.corners[1] = corner_plus(cube.corners[7]);
    cube_trans.corners[7] = corner_minus(cube.corners[4]);
    return cube_trans;
}
pub fn rotate_f(cube: Cube) -> Cube {
    let mut cube_trans = cube.clone();
    cube_trans.edges[0] = edge_flip(cube.edges[9]);
    cube_trans.edges[8] = edge_flip(cube.edges[0]);
    cube_trans.edges[4] = edge_flip(cube.edges[8]);
    cube_trans.edges[9] = edge_flip(cube.edges[4]);

    cube_trans.corners[4] = corner_minus(cube.corners[0]);
    cube_trans.corners[5] = corner_plus(cube.corners[4]);
    cube_trans.corners[3] = corner_minus(cube.corners[5]);
    cube_trans.corners[0] = corner_plus(cube.corners[3]);
    return cube_trans;
}
pub fn rotate_fi(cube: Cube) -> Cube {
    let mut cube_trans = cube.clone();
    cube_trans.edges[9] = edge_flip(cube.edges[0]);
    cube_trans.edges[0] = edge_flip(cube.edges[8]);
    cube_trans.edges[8] = edge_flip(cube.edges[4]);
    cube_trans.edges[4] = edge_flip(cube.edges[9]);

    cube_trans.corners[0] = corner_plus(cube.corners[4]);
    cube_trans.corners[4] = corner_minus(cube.corners[5]);
    cube_trans.corners[5] = corner_plus(cube.corners[3]);
    cube_trans.corners[3] = corner_minus(cube.corners[0]);
    return cube_trans;
}

pub fn rotate_b(cube: Cube) -> Cube {
    let mut cube_trans = cube.clone();
    cube_trans.edges[11] = edge_flip(cube.edges[2]);
    cube_trans.edges[6] = edge_flip(cube.edges[11]);
    cube_trans.edges[10] = edge_flip(cube.edges[6]);
    cube_trans.edges[2] = edge_flip(cube.edges[10]);

    cube_trans.corners[2] = corner_plus(cube.corners[1]);
    cube_trans.corners[6] = corner_minus(cube.corners[2]);
    cube_trans.corners[7] = corner_plus(cube.corners[6]);
    cube_trans.corners[1] = corner_minus(cube.corners[7]);
    return cube_trans;
}

pub fn rotate_bi(cube: Cube) -> Cube {
    let mut cube_trans = cube.clone();
    cube_trans.edges[2] = edge_flip(cube.edges[11]);
    cube_trans.edges[11] = edge_flip(cube.edges[6]);
    cube_trans.edges[6] = edge_flip(cube.edges[10]);
    cube_trans.edges[10] = edge_flip(cube.edges[2]);

    cube_trans.corners[1] = corner_minus(cube.corners[2]);
    cube_trans.corners[2] = corner_plus(cube.corners[6]);
    cube_trans.corners[6] = corner_minus(cube.corners[7]);
    cube_trans.corners[7] = corner_plus(cube.corners[1]);
    return cube_trans;
}

pub fn rotate_l(cube: Cube) -> Cube {
    let mut cube_trans = cube.clone();
    cube_trans.edges[9] = cube.edges[3];
    cube_trans.edges[7] = cube.edges[9];
    cube_trans.edges[11] = cube.edges[7];
    cube_trans.edges[3] = cube.edges[11];

    cube_trans.corners[5] = corner_minus(cube.corners[3]);
    cube_trans.corners[6] = corner_plus(cube.corners[5]);
    cube_trans.corners[2] = corner_minus(cube.corners[6]);
    cube_trans.corners[3] = corner_plus(cube.corners[2]);
    return cube_trans;
}

pub fn rotate_li(cube: Cube) -> Cube {
    let mut cube_trans = cube.clone();

    cube_trans.edges[3] = cube.edges[9];
    cube_trans.edges[9] = cube.edges[7];
    cube_trans.edges[7] = cube.edges[11];
    cube_trans.edges[11] = cube.edges[3];

    cube_trans.corners[3] = corner_plus(cube.corners[5]);
    cube_trans.corners[5] = corner_minus(cube.corners[6]);
    cube_trans.corners[6] = corner_plus(cube.corners[2]);
    cube_trans.corners[2] = corner_minus(cube.corners[3]);
    return cube_trans;
}

pub fn rotate_d(cube: Cube) -> Cube {
    let mut cube_trans = cube.clone();
    cube_trans.edges[5] = cube.edges[4];
    cube_trans.edges[6] = cube.edges[5];
    cube_trans.edges[7] = cube.edges[6];
    cube_trans.edges[4] = cube.edges[7];

    cube_trans.corners[7] = cube.corners[4];
    cube_trans.corners[6] = cube.corners[7];
    cube_trans.corners[5] = cube.corners[6];
    cube_trans.corners[4] = cube.corners[5];
    return cube_trans;
}

pub fn rotate_di(cube: Cube) -> Cube {
    let mut cube_trans = cube.clone();

    cube_trans.edges[4] = cube.edges[5];
    cube_trans.edges[5] = cube.edges[6];
    cube_trans.edges[6] = cube.edges[7];
    cube_trans.edges[7] = cube.edges[4];

    cube_trans.corners[4] = cube.corners[7];
    cube_trans.corners[7] = cube.corners[6];
    cube_trans.corners[6] = cube.corners[5];
    cube_trans.corners[5] = cube.corners[4];
    return cube_trans;
}

pub fn print_turns(turns: Vec<CubeTurn>) {
    let mut turn_list = String::new();
    for turn in turns {
        match turn {
            CubeTurn::Up => turn_list += "U ",
            CubeTurn::UpI => turn_list += "U' ",
            CubeTurn::Up2 => turn_list += "U2 ",
            CubeTurn::Right => turn_list += "R ",
            CubeTurn::RightI => turn_list += "R' ",
            CubeTurn::Right2 => turn_list += "R2 ",
            CubeTurn::Front => turn_list += "F ",
            CubeTurn::FrontI => turn_list += "F' ",
            CubeTurn::Front2 => turn_list += "F2 ",
            CubeTurn::Left => turn_list += "L ",
            CubeTurn::LeftI => turn_list += "L' ",
            CubeTurn::Left2 => turn_list += "L2 ",
            CubeTurn::Down => turn_list += "D ",
            CubeTurn::DownI => turn_list += "D' ",
            CubeTurn::Down2 => turn_list += "D2 ",
            CubeTurn::Back => turn_list += "B ",
            CubeTurn::BackI => turn_list += "B' ",
            CubeTurn::Back2 => turn_list += "B2 ",
            _ => {}
        }
    }
    println!("alg: {}", turn_list);

}
pub fn notation2turns(alg: &str) -> Vec<CubeTurn> {
    let mut turn_list = Vec::new();
    let turns = alg.split_whitespace();

    for turn in turns {
        match turn.trim() {
            "U" => turn_list.push(CubeTurn::Up),
            "U'" => turn_list.push(CubeTurn::UpI),
            "U2" => turn_list.push(CubeTurn::Up2),
            "R" => turn_list.push(CubeTurn::Right),
            "R'" => turn_list.push(CubeTurn::RightI),
            "R2" => turn_list.push(CubeTurn::Right2),
            "F" => turn_list.push(CubeTurn::Front),
            "F'" => turn_list.push(CubeTurn::FrontI),
            "F2" => turn_list.push(CubeTurn::Front2),
            "L" => turn_list.push(CubeTurn::Left),
            "L'" => turn_list.push(CubeTurn::LeftI),
            "L2" => turn_list.push(CubeTurn::Left2),
            "D" => turn_list.push(CubeTurn::Down),
            "D'" => turn_list.push(CubeTurn::DownI),
            "D2" => turn_list.push(CubeTurn::Down2),
            "B" => turn_list.push(CubeTurn::Back),
            "B'" => turn_list.push(CubeTurn::BackI),
            "B2" => turn_list.push(CubeTurn::Back2),
            _ => {}
        }
    }
    return turn_list;
}

pub fn apply_turns(cube: Cube, alg: Vec<CubeTurn>) -> Cube {
    let mut cube_trans = cube;
    for turn in alg.iter() {
        match turn {
            CubeTurn::Up => cube_trans = rotate_u(cube_trans),
            CubeTurn::UpI => cube_trans = rotate_ui(cube_trans),
            CubeTurn::Up2 => {
                cube_trans = rotate_u(cube_trans);
                cube_trans = rotate_u(cube_trans);
            },
            CubeTurn::Right => cube_trans = rotate_r(cube_trans),
            CubeTurn::RightI => cube_trans = rotate_ri(cube_trans),
            CubeTurn::Right2 => {
                cube_trans = rotate_r(cube_trans);
                cube_trans = rotate_r(cube_trans);
            },
            CubeTurn::Front => cube_trans = rotate_f(cube_trans),
            CubeTurn::FrontI => cube_trans = rotate_fi(cube_trans),
            CubeTurn::Front2 => {
                cube_trans = rotate_f(cube_trans);
                cube_trans = rotate_f(cube_trans);
            },
            CubeTurn::Left => cube_trans = rotate_l(cube_trans),
            CubeTurn::LeftI => cube_trans = rotate_li(cube_trans),
            CubeTurn::Left2 => {
                cube_trans = rotate_l(cube_trans);
                cube_trans = rotate_l(cube_trans);
            },
            CubeTurn::Down => cube_trans = rotate_d(cube_trans),
            CubeTurn::DownI => cube_trans = rotate_di(cube_trans),
            CubeTurn::Down2 => {
                cube_trans = rotate_d(cube_trans);
                cube_trans = rotate_d(cube_trans);
            },
            CubeTurn::Back => cube_trans = rotate_b(cube_trans),
            CubeTurn::BackI => cube_trans = rotate_bi(cube_trans),
            CubeTurn::Back2 => {
                cube_trans = rotate_b(cube_trans);
                cube_trans = rotate_b(cube_trans);
            },
        }
    }
    return cube_trans;
}

pub fn apply_turn(cube: Cube, turn: CubeTurn) -> Cube {
    let mut cube_trans = cube;
    match turn {
            CubeTurn::Up => cube_trans = rotate_u(cube_trans),
            CubeTurn::UpI => cube_trans = rotate_ui(cube_trans),
            CubeTurn::Up2 => {
                cube_trans = rotate_u(cube_trans);
                cube_trans = rotate_u(cube_trans);
            },
            CubeTurn::Right => cube_trans = rotate_r(cube_trans),
            CubeTurn::RightI => cube_trans = rotate_ri(cube_trans),
            CubeTurn::Right2 => {
                cube_trans = rotate_r(cube_trans);
                cube_trans = rotate_r(cube_trans);
            },
            CubeTurn::Front => cube_trans = rotate_f(cube_trans),
            CubeTurn::FrontI => cube_trans = rotate_fi(cube_trans),
            CubeTurn::Front2 => {
                cube_trans = rotate_f(cube_trans);
                cube_trans = rotate_f(cube_trans);
            },
            CubeTurn::Left => cube_trans = rotate_l(cube_trans),
            CubeTurn::LeftI => cube_trans = rotate_li(cube_trans),
            CubeTurn::Left2 => {
                cube_trans = rotate_l(cube_trans);
                cube_trans = rotate_l(cube_trans);
            },
            CubeTurn::Down => cube_trans = rotate_d(cube_trans),
            CubeTurn::DownI => cube_trans = rotate_di(cube_trans),
            CubeTurn::Down2 => {
                cube_trans = rotate_d(cube_trans);
                cube_trans = rotate_d(cube_trans);
            },
            CubeTurn::Back => cube_trans = rotate_b(cube_trans),
            CubeTurn::BackI => cube_trans = rotate_bi(cube_trans),
            CubeTurn::Back2 => {
                cube_trans = rotate_b(cube_trans);
                cube_trans = rotate_b(cube_trans);
            },
    }
    return cube_trans;
}
//trying to make some simple pruning algorithms for
//possibly to solve cube later
pub fn edges_distance_g0( cube: Cube ) -> u32
{
      //UF UR UB   UL  DF  DR DB DL FR FL BR BL 
    /*let edge_lookup : [u32; 288] = [ 
        0, 1,  1,  1,  1,  2, 2, 2, 2, 2, 2, 2, //UF
        3, 2,  3,  2,  3,  2, 3, 2, 1, 1, 2, 2, //UF flipped

        1, 0,  1,  1,  2,  1, 2, 2, 1, 2, 1, 2, //UR
        2, 3,  2,  3,  2,  3, 2, 3, 2, 2, 2, 2, //UR flipped

        1, 1,  0,  1,  2,  2, 1, 2, 2, 2, 2, 2, //UB
        3, 2,  3,  2,  3,  2, 3, 2, 2, 2, 2, 2, //UB flipped

        1, 1,  1,  0,  2,  2, 2, 1, 2, 1, 2, 1, //UL
        2, 3,  2,  3,  2,  3, 2, 3, 2, 2, 2, 2, //UL flipped

        1, 2,  2,  2,  0,  1, 1, 1, 2, 2, 2, 2, //DF
        3, 2,  3,  2,  3,  2, 3, 2, 1, 1, 2, 2, //DF flipped

        2, 1,  2,  2,  1,  0, 1, 1, 1, 2, 1, 2, //DR
        2, 3,  2,  3,  2,  3, 2, 3, 2, 2, 2, 2, //DR flipped

        2, 2,  1,  2,  1,  1, 0, 1, 2, 2, 2, 2, //DB
        3, 2,  3,  2,  3,  2, 3, 2, 2, 2, 2, 2, //DB flipped

        2, 2,  2,  1,  1,  1, 1, 0, 2, 1, 2, 1, //DL
        2, 3,  2,  3,  2,  3, 2, 3, 2, 2, 2, 2, //DL flipped

        2, 1,  2,  2,  2,  1, 2, 2, 0, 1, 1, 2, //FR
        1, 2,  2,  2,  1,  2, 2, 2, 3, 3, 3, 3, //FR flipped

        2, 2,  2,  1,  2,  2, 2, 1, 1, 0, 2, 1, //FL
        1, 2,  2,  2,  1,  2, 2, 2, 3, 3, 3, 3, //FL flipped

        2, 1,  2,  2,  2,  1, 2, 2, 1, 2, 0, 1, //BR
        2, 2,  1,  2,  2,  2, 1, 2, 3, 3, 3, 3, //BR flipped

        2, 2,  2,  1,  2,  2, 2, 1, 2, 1, 1, 0, //BL
        2, 2,  1,  2,  2,  2, 1, 2, 3, 3, 3, 3, //BL flipped
        ];*/
    let edge_lookup : [u32; 288] = [ 
        0, 0,  0,  0,  0,  0, 0, 0, 1, 1, 1, 1, //UF
        1, 2,  1,  2,  1,  2, 1, 2, 1, 1, 1, 1, //UF flipped

        0, 0,  0,  0,  0,  0, 0, 0, 1, 1, 1, 1, //UR
        1, 2,  1,  2,  1,  2, 1, 2, 1, 1, 1, 1, //UR flipped
        
        0, 0,  0,  0,  0,  0, 0, 0, 1, 1, 1, 1, //UB
        1, 2,  1,  2,  1,  2, 1, 2, 1, 1, 1, 1, //UB flipped

        0, 0,  0,  0,  0,  0, 0, 0, 1, 1, 1, 1, //UL
        1, 2,  1,  2,  1,  2, 1, 2, 1, 1, 1, 1, //UL flipped

        0, 0,  0,  0,  0,  0, 0, 0, 1, 1, 1, 1, //DF
        1, 2,  1,  2,  1,  2, 1, 2, 1, 1, 1, 1, //DF flipped

        0, 0,  0,  0,  0,  0, 0, 0, 1, 1, 1, 1, //DR
        1, 2,  1,  2,  1,  2, 1, 2, 1, 1, 1, 1, //DR flipped
        
        0, 0,  0,  0,  0,  0, 0, 0, 1, 1, 1, 1, //DB
        1, 2,  1,  2,  1,  2, 1, 2, 1, 1, 1, 1, //DB flipped

        0, 0,  0,  0,  0,  0, 0, 0, 1, 1, 1, 1, //DL
        1, 2,  1,  2,  1,  2, 1, 2, 1, 1, 1, 1, //DL flipped

        2, 1,  2,  1,  2,  1, 2, 1, 0, 0, 0, 0, //FR
        1, 2,  1,  2,  1,  2, 1, 2, 3, 3, 3, 3, //FR flipped

        2, 1,  2,  1,  2,  1, 2, 1, 0, 0, 0, 0, //FL
        1, 2,  1,  2,  1,  2, 1, 2, 3, 3, 3, 3, //FL flipped

        2, 1,  2,  1,  2,  1, 2, 1, 0, 0, 0, 0, //BR
        1, 2,  1,  2,  1,  2, 1, 2, 3, 3, 3, 3, //BR flipped

        2, 1,  2,  1,  2,  1, 2, 1, 0, 0, 0, 0, //BL
        1, 2,  1,  2,  1,  2, 1, 2, 3, 3, 3, 3, //BL flipped
        ];
    let mut distance : u32 = 0;
    let mut lookup_index : u16 = 0;
    for e in cube.edges.iter() {
        if e.id > 0 {
            let mut offset : usize = e.id.into();
            offset -= 1;
            offset *= 24;
            offset += usize::from(lookup_index);
            if e.orient != 0 {
                offset += 12; 
            } 
            distance += edge_lookup[offset];
        }
        lookup_index += 1;
    }
    return distance;
}
pub fn corners_distance_g0( cube: Cube ) -> u32
{
    let mut distance : u32= 0;
    for e in cube.corners.iter() {
        if e.id > 0 {
            if e.orient != 0 {
                distance += 1;
            }
        }
    }
    return distance;
}
//this is my simple heuristic function for solving G1 state
//trying to estimate
pub fn distance_g0( cube: Cube ) -> u32
{
    return edges_distance_g0(cube);//+ corners_distance_g0(cube);
}

pub fn distance_g1( cube: Cube ) -> u32
{
    return edges_distance_g0(cube) + corners_distance_g0(cube);
}


//## Pseudo
//For each solvestates 
//    apply all valid turns
//        remove turn from valid (all similar U U' U2)
//        add all paraller turns to valid ( for U -> R L B F)
//        keep opposite turn if already on list
//    estimate heuristic distance
//    push all these new states into new container state+1
//
pub fn apply_valid_turns_g0( state: SolveState) -> Vec<SolveState> 
{
    let mut new_states : Vec<SolveState> = Vec::new();
    for turn in state.valid_turns.iter() {
        let mut t : CubeTurn = turn.clone();
        let cube_trans = apply_turn(state.state, t);
        let distance = distance_g0(cube_trans);
        let mut next_turns : Vec<CubeTurn> = Vec::new();
        let mut alg_n : Vec<CubeTurn> = Vec::new();
        alg_n = state.alg.to_vec();
        alg_n.push(*turn);
        //this looks horrible.. clean up!
        match turn {
            CubeTurn::Up | CubeTurn::UpI | CubeTurn::Up2 => {
                next_turns = vec![CubeTurn::Right, CubeTurn::RightI, 
                    CubeTurn::Left, CubeTurn::LeftI, 
                    CubeTurn::Back, CubeTurn::BackI, 
                    CubeTurn::Front, CubeTurn::FrontI, ];
                if state.valid_turns.contains(&CubeTurn::Down) {
                    next_turns.push(CubeTurn::Down);
                    next_turns.push(CubeTurn::DownI);
                }
            }
            CubeTurn::Down | CubeTurn::DownI | CubeTurn::Down2 => {
                next_turns = vec![CubeTurn::Right, CubeTurn::RightI, 
                    CubeTurn::Left, CubeTurn::LeftI, 
                    CubeTurn::Back, CubeTurn::BackI, 
                    CubeTurn::Front, CubeTurn::FrontI, ];
                if state.valid_turns.contains(&CubeTurn::Up) {
                    next_turns.push(CubeTurn::Up);
                    next_turns.push(CubeTurn::UpI);
                }
            }
            CubeTurn::Right | CubeTurn::RightI | CubeTurn::Right2 => {
                next_turns = vec![CubeTurn::Up, CubeTurn::UpI, 
                    CubeTurn::Down, CubeTurn::DownI, 
                    CubeTurn::Back, CubeTurn::BackI, 
                    CubeTurn::Front, CubeTurn::FrontI ];
                if state.valid_turns.contains(&CubeTurn::Left) {
                    next_turns.push(CubeTurn::Left);
                    next_turns.push(CubeTurn::LeftI);
                }
            }
            CubeTurn::Left | CubeTurn::LeftI | CubeTurn::Left2 => {
                next_turns = vec![CubeTurn::Up, CubeTurn::UpI, 
                    CubeTurn::Down, CubeTurn::DownI, 
                    CubeTurn::Back, CubeTurn::BackI, 
                    CubeTurn::Front, CubeTurn::FrontI ];
                if state.valid_turns.contains(&CubeTurn::Right) {
                    next_turns.push(CubeTurn::Right);
                    next_turns.push(CubeTurn::RightI);
                }
            }
            CubeTurn::Back | CubeTurn::BackI | CubeTurn::Back2 => {
                next_turns = vec![CubeTurn::Up, CubeTurn::UpI, 
                    CubeTurn::Down, CubeTurn::DownI, 
                    CubeTurn::Right, CubeTurn::RightI, 
                    CubeTurn::Left, CubeTurn::LeftI ];
                if state.valid_turns.contains(&CubeTurn::Front) {
                    next_turns.push(CubeTurn::Front);
                    next_turns.push(CubeTurn::FrontI);
                }
            }
            CubeTurn::Front | CubeTurn::FrontI | CubeTurn::Front2 => {
                next_turns = vec![CubeTurn::Up, CubeTurn::UpI, 
                    CubeTurn::Down, CubeTurn::DownI, 
                    CubeTurn::Right, CubeTurn::RightI, 
                    CubeTurn::Left, CubeTurn::LeftI ];
                if state.valid_turns.contains(&CubeTurn::Back) {
                    next_turns.push(CubeTurn::Back);
                    next_turns.push(CubeTurn::BackI);
                }
            }
        }
        let state_n = SolveState {
                dist: distance,
                alg: alg_n,
                state: cube_trans,
                valid_turns: next_turns,
        };
        new_states.push(state_n);
    }
    return new_states;

}
pub fn apply_valid_turns_g1( state: SolveState) -> Vec<SolveState> 
{
    let mut new_states : Vec<SolveState> = Vec::new();
    for turn in state.valid_turns.iter() {
        let mut t : CubeTurn = turn.clone();
        let cube_trans = apply_turn(state.state, t);
        let distance = distance_g1(cube_trans);
        let mut next_turns : Vec<CubeTurn> = Vec::new();
        let mut alg_n : Vec<CubeTurn> = Vec::new();
        alg_n = state.alg.to_vec();
        alg_n.push(*turn);
        //this looks horrible.. clean up!
        match turn {
            CubeTurn::Up | CubeTurn::UpI | CubeTurn::Up2 => {
                next_turns = vec![CubeTurn::Right, CubeTurn::RightI, 
                    CubeTurn::Left, CubeTurn::LeftI, 
                    CubeTurn::Back2,
                    CubeTurn::Front2 ];
                if state.valid_turns.contains(&CubeTurn::Down) {
                    next_turns.push(CubeTurn::Down);
                    next_turns.push(CubeTurn::DownI);
                }
            }
            CubeTurn::Down | CubeTurn::DownI | CubeTurn::Down2 => {
                next_turns = vec![CubeTurn::Right, CubeTurn::RightI, 
                    CubeTurn::Left, CubeTurn::LeftI, 
                    CubeTurn::Back2,
                    CubeTurn::Front2 ];
                if state.valid_turns.contains(&CubeTurn::Up) {
                    next_turns.push(CubeTurn::Up);
                    next_turns.push(CubeTurn::UpI);
                }
            }
            CubeTurn::Right | CubeTurn::RightI | CubeTurn::Right2 => {
                next_turns = vec![CubeTurn::Up, CubeTurn::UpI, 
                    CubeTurn::Down, CubeTurn::DownI, 
                    CubeTurn::Back2,
                    CubeTurn::Front2 ];
                if state.valid_turns.contains(&CubeTurn::Left) {
                    next_turns.push(CubeTurn::Left);
                    next_turns.push(CubeTurn::LeftI);
                }
            }
            CubeTurn::Left | CubeTurn::LeftI | CubeTurn::Left2 => {
                next_turns = vec![CubeTurn::Up, CubeTurn::UpI, 
                    CubeTurn::Down, CubeTurn::DownI, 
                    CubeTurn::Back2, 
                    CubeTurn::Front2 ];
                if state.valid_turns.contains(&CubeTurn::Right) {
                    next_turns.push(CubeTurn::Right);
                    next_turns.push(CubeTurn::RightI);
                }
            }
            CubeTurn::Back | CubeTurn::BackI | CubeTurn::Back2 => {
                next_turns = vec![CubeTurn::Up, CubeTurn::UpI, 
                    CubeTurn::Down, CubeTurn::DownI, 
                    CubeTurn::Right, CubeTurn::RightI, 
                    CubeTurn::Left, CubeTurn::LeftI ];
                if state.valid_turns.contains(&CubeTurn::Front2) {
                    next_turns.push(CubeTurn::Front2);
                }
            }
            CubeTurn::Front | CubeTurn::FrontI | CubeTurn::Front2 => {
                next_turns = vec![CubeTurn::Up, CubeTurn::UpI, 
                    CubeTurn::Down, CubeTurn::DownI, 
                    CubeTurn::Right, CubeTurn::RightI, 
                    CubeTurn::Left, CubeTurn::LeftI ];
                if state.valid_turns.contains(&CubeTurn::Back2) {
                    next_turns.push(CubeTurn::Back2);
                }
            }
        }
        let state_n = SolveState {
                dist: distance,
                alg: alg_n,
                state: cube_trans,
                valid_turns: next_turns,
        };
        new_states.push(state_n);
    }
    return new_states;

}
pub fn solve_g0( mut max_iter : u16, mut state: Vec<SolveState>) -> Vec<SolveState> 
{
    let mut iterations = 0;
    println!("solving G0 state");
    while iterations < max_iter {
        println!("iterations {} ", iterations);
        let mut new_states : Vec<SolveState> = Vec::new();
        for branch in state.iter() {
            let mut current = branch.clone();
            let mut branch_step = apply_valid_turns_g0(current);
            new_states.append(&mut branch_step);
        }
        new_states.sort_by(|a,b| a.dist.cmp(&b.dist));
        state = new_states;
        iterations += 1;
        if iterations % 4 == 0 {
            if state.len() > 1024{
                state.split_off(1024);
            }
        }
        if state[0].dist == 0 {
            break;
        }
    }
    println!("cleaning up unsolved");
    let mut index = 0;
    for i in state.iter() {
        if i.dist != 0 {
            break;
        }
        index += 1;
    }
    state.split_off(index);
    iterations = 0;
    println!("found {} results", index);
    println!("solving G1 state");
    while iterations < max_iter {
        println!("iterations {} ", iterations);
        let mut new_states : Vec<SolveState> = Vec::new();
        for branch in state.iter() {
            let mut current = branch.clone();
            let mut branch_step = apply_valid_turns_g1(current);
            new_states.append(&mut branch_step);
        }
        new_states.sort_by(|a,b| a.dist.cmp(&b.dist));
        state = new_states;
        max_iter -= 1;
        if max_iter % 4 == 0 {
            if state.len() > 1024{
                state.split_off(1024);
            }
        }
        if state[0].dist == 0 {
            break;
        }
        iterations += 1;
    }
    return state;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotation_up() {
        let u = "UR UB UL UF DF DR DB DL FR FL BR BL URB UBL ULF UFR DRF DFL DLB DBR ";
        let cube = solved_cube();
        let cube_rotated = rotate_u(cube);
        let pos = pos_as_string(cube_rotated);
        assert_eq!(u, pos);
    }
    #[test]
    fn rotation_up_inv() {
        let u = "UL UF UR UB DF DR DB DL FR FL BR BL ULF UFR URB UBL DRF DFL DLB DBR ";
        let cube = solved_cube();
        let cube_rotated = rotate_ui(cube);
        let pos = pos_as_string(cube_rotated);
        assert_eq!(u, pos);
    }
    #[test]
    fn rotation_down() {
        let u = "UF UR UB UL DL DF DR DB FR FL BR BL UFR URB UBL ULF DFL DLB DBR DRF ";
        let cube = solved_cube();
        let cube_rotated = rotate_d(cube);
        let pos = pos_as_string(cube_rotated);
        assert_eq!(u, pos);
    }
    #[test]
    fn rotation_down_inv() {
        let u = "UF UR UB UL DR DB DL DF FR FL BR BL UFR URB UBL ULF DBR DRF DFL DLB ";
        let cube = solved_cube();
        let cube_rotated = rotate_di(cube);
        let pos = pos_as_string(cube_rotated);
        assert_eq!(u, pos);
    }
    #[test]
    fn rotation_right() {
        let u = "UF FR UB UL DF BR DB DL DR FL UR BL -DRF +UFR UBL ULF +DBR DFL DLB -URB ";
        let cube = solved_cube();
        let cube_rotated = rotate_r(cube);
        let pos = pos_as_string(cube_rotated);
        assert_eq!(u, pos);
    }
    #[test]
    fn rotation_right_inv() {
        let u = "UF BR UB UL DF FR DB DL UR FL DR BL -URB +DBR UBL ULF +UFR DFL DLB -DRF ";
        let cube = solved_cube();
        let cube_rotated = rotate_ri(cube);
        let pos = pos_as_string(cube_rotated);
        assert_eq!(u, pos);
    }
    #[test]
    fn rotation_left() {
        let u = "UF UR UB BL DF DR DB FL FR UL BR DL UFR URB -DLB +UBL DRF -ULF +DFL DBR ";
        let cube = solved_cube();
        let cube_rotated = rotate_l(cube);
        let pos = pos_as_string(cube_rotated);
        assert_eq!(u, pos);
    }
    #[test]
    fn rotation_left_inv() {
        let u = "UF UR UB FL DF DR DB BL FR DL BR UL UFR URB -ULF +DFL DRF -DLB +UBL DBR ";
        let cube = solved_cube();
        let cube_rotated = rotate_li(cube);
        let pos = pos_as_string(cube_rotated);
        assert_eq!(u, pos);
    }
    #[test]
    fn rotation_back() {
        let u = "UF UR -BR UL DF DR -BL DL FR FL -DB -UB UFR -DBR +URB ULF DRF DFL -UBL +DLB ";
        let cube = solved_cube();
        let cube_rotated = rotate_b(cube);
        let pos = pos_as_string(cube_rotated);
        assert_eq!(u, pos);
    }
    #[test]
    fn rotation_back_inv() {
        let u = "UF UR -BL UL DF DR -BR DL FR FL -UB -DB UFR -UBL +DLB ULF DRF DFL -DBR +URB ";
        let cube = solved_cube();
        let cube_rotated = rotate_bi(cube);
        let pos = pos_as_string(cube_rotated);
        assert_eq!(u, pos);
    }
    #[test]
    fn rotation_front() {
        let u = "-FL UR UB UL -FR DR DB DL -UF -DF BR BL +ULF URB UBL -DFL -UFR +DRF DLB DBR ";
        let cube = solved_cube();
        let cube_rotated = rotate_f(cube);
        let pos = pos_as_string(cube_rotated);
        assert_eq!(u, pos);
    }
    #[test]
    fn rotation_front_inv() {
        let u = "-FR UR UB UL -FL DR DB DL -DF -UF BR BL +DRF URB UBL -UFR -DFL +ULF DLB DBR ";
        let cube = solved_cube();
        let cube_rotated = rotate_fi(cube);
        let pos = pos_as_string(cube_rotated);
        assert_eq!(u, pos);
    }
}
