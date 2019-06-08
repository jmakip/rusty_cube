extern crate rusty_cube;



fn main() {
    let all = "R R' L L' U U' D D' B B' F' F";
    let all_turns = rusty_cube::notation2turns(all);
    let alg = "F U B2 L' B2 L2 D2 L' D' F2 U2 B' L2 D2 B' D2 L2 F L2 F";
    //let alg = "D' L D2 B' U' L2 B' D2 L' F U B2 D L2 U' B2 D R2 L2 U B2";
    //let alg = "F2 L2 D2 U2 B U2 B2 D2 R2 D2 L' D' R F U B' U L' D' B'";
    let cube = rusty_cube::solved_cube();
    print!("zero: ");
    rusty_cube::print_cubies(cube);

    let turns = rusty_cube::notation2turns(alg);
    let cube_trans = rusty_cube::apply_turns(cube, turns);
    print!("alg: ");
    rusty_cube::print_cubies(cube_trans);


    let distance = rusty_cube::distance_g0(cube_trans);

    println!("G0 distance: {} ", distance);

    let mut max_iter : u16 = 20;
    let start : rusty_cube::SolveState = rusty_cube::SolveState {
        dist : 20,
        alg: Vec::new(),
        state: cube_trans,
        valid_turns: all_turns,
    };
    let mut state = vec![start];
    let mut solved = rusty_cube::solve_g0(  max_iter ,state);

    for states in solved.iter() {
        println!("distance : {}", states.dist);
        let mut turnee= states.alg.to_vec();
        rusty_cube::print_turns(turnee);

    }
}
