extern crate rusty_cube;
fn main() {
    let mut cube = rusty_cube::solved_cube();
    print!("solved: ");
    rusty_cube::print_cubies(cube);

    let mut cube_trans = rusty_cube::rotate_u(cube);
    print!("U: ");
    rusty_cube::print_cubies(cube_trans);

    print!("U': ");
    cube_trans = rusty_cube::rotate_ui(cube);
    rusty_cube::print_cubies(cube_trans);

    print!("R: ");
    cube_trans = rusty_cube::rotate_r(cube);
    rusty_cube::print_cubies(cube_trans);
    print!("R': ");
    cube_trans = rusty_cube::rotate_ri(cube);
    rusty_cube::print_cubies(cube_trans);

    print!("D: ");
    cube_trans = rusty_cube::rotate_d(cube);
    rusty_cube::print_cubies(cube_trans);
    print!("D': ");
    cube_trans = rusty_cube::rotate_di(cube);
    rusty_cube::print_cubies(cube_trans);

    print!("L: ");
    cube_trans = rusty_cube::rotate_l(cube);
    rusty_cube::print_cubies(cube_trans);
    print!("L': ");
    cube_trans = rusty_cube::rotate_li(cube);
    rusty_cube::print_cubies(cube_trans);

    print!("F: ");
    cube_trans = rusty_cube::rotate_f(cube);
    rusty_cube::print_cubies(cube_trans);
    print!("F': ");
    cube_trans = rusty_cube::rotate_fi(cube);
    rusty_cube::print_cubies(cube_trans);

    print!("B: ");
    cube_trans = rusty_cube::rotate_b(cube);
    rusty_cube::print_cubies(cube_trans);
    print!("B': ");
    cube_trans = rusty_cube::rotate_bi(cube);
    rusty_cube::print_cubies(cube_trans);
}
