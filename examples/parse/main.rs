extern crate rusty_cube;

enum CubeTurn {
    UP,
    UP_I,
    RIGHT,
    RIGHT_I,
    LEFT,
    LEFT_I,
    DOWN,
    DOWN_I,
    BACK,
    BACK_I,
}

fn notation2turns(alg: &str) -> Vec<CubeTurn> {
    let mut turn_list = Vec::new();
    let turns = alg.split_whitespace();

    for turn in turns {
        match turn.trim() {
            "U" => turn_list.push(CubeTurn::UP),
            "U'" => turn_list.push(CubeTurn::UP_I),
            "R" => turn_list.push(CubeTurn::RIGHT),
            "R'" => turn_list.push(CubeTurn::RIGHT_I),
            "L" => turn_list.push(CubeTurn::LEFT),
            "L'" => turn_list.push(CubeTurn::LEFT_I),
            "D" => turn_list.push(CubeTurn::DOWN),
            "D'" => turn_list.push(CubeTurn::DOWN_I),
            "B" => turn_list.push(CubeTurn::BACK),
            "B'" => turn_list.push(CubeTurn::BACK_I),
            _ => {}
        }
    }
    return turn_list;
}

fn main() {
    let alg = "R U' L' U R' U' L ";
    let cube = rusty_cube::solved_cube();
    let mut cube_trans = cube;
    print!("zero: ");
    rusty_cube::print_cubies(cube);

    //let turns = alg.split_whitespace();
    /*
    for turn in turns {
        match turn.trim() {
        "U" => cube_trans = rusty_cube::rotate_u(cube_trans),
        "U'" => cube_trans = rusty_cube::rotate_ui(cube_trans),
        "R" => cube_trans = rusty_cube::rotate_r(cube_trans),
        "R'" => cube_trans = rusty_cube::rotate_ri(cube_trans),
        "L" => cube_trans = rusty_cube::rotate_l(cube_trans),
        "L'" => cube_trans = rusty_cube::rotate_li(cube_trans),
        "D" => cube_trans = rusty_cube::rotate_d(cube_trans),
        "D'" => cube_trans = rusty_cube::rotate_di(cube_trans),
        "B" => cube_trans = rusty_cube::rotate_b(cube_trans),
        "B'" => cube_trans = rusty_cube::rotate_bi(cube_trans),
        _ => {},
        }
    }

    */
    let turns = notation2turns(alg);
    for turn in turns.iter() {
        match turn {
            CubeTurn::UP => cube_trans = rusty_cube::rotate_u(cube_trans),
            CubeTurn::UP_I => cube_trans = rusty_cube::rotate_ui(cube_trans),
            CubeTurn::RIGHT => cube_trans = rusty_cube::rotate_r(cube_trans),
            CubeTurn::RIGHT_I => cube_trans = rusty_cube::rotate_ri(cube_trans),
            CubeTurn::LEFT => cube_trans = rusty_cube::rotate_l(cube_trans),
            CubeTurn::LEFT_I => cube_trans = rusty_cube::rotate_li(cube_trans),
            CubeTurn::DOWN => cube_trans = rusty_cube::rotate_d(cube_trans),
            CubeTurn::DOWN_I => cube_trans = rusty_cube::rotate_di(cube_trans),
            CubeTurn::BACK => cube_trans = rusty_cube::rotate_b(cube_trans),
            CubeTurn::BACK_I => cube_trans = rusty_cube::rotate_bi(cube_trans),
            _ => {}
        }
    }
    print!("alg: ");
    rusty_cube::print_cubies(cube_trans);
}
