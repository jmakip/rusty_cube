extern crate rusty_cube;
fn main() {
    let mut cube = rusty_cube::solved_cube();
    rusty_cube::print_cubies(cube);
    println!("solved");
    let mut cube_trans = rusty_cube::rotate_d(cube);
    rusty_cube::print_cubies(cube_trans);
    cube_trans = rusty_cube::rotate_di(cube_trans);
    rusty_cube::print_cubies(cube_trans);
}
