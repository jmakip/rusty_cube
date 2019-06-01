#[derive(Clone, Copy)]
pub struct Edge {
    id: u32,
    orient: i8,
}
#[derive(Clone, Copy)]
pub struct Corner {
    id: u32,
    orient: i8,
}
#[derive(Clone, Copy)]
pub struct Cube {
    edges: [Edge; 12],
    corners: [Corner; 8],
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
