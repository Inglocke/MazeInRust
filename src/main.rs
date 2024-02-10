extern crate minifb;

use std::iter::FlatMap;

use minifb::{Key, Window, WindowOptions};
use rand::seq::SliceRandom;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn get_neighbors(coord: Coord, rows: usize, cols: usize) -> Vec<Coord> {
    let mut neighbors = Vec::new();
    if coord.x > 0 {
        neighbors.push(Coord::new(coord.x - 1, coord.y));
    }
    if coord.x < cols - 1 {
        neighbors.push(Coord::new(coord.x + 1, coord.y));
    }
    if coord.y > 0 {
        neighbors.push(Coord::new(coord.x, coord.y - 1));
    }
    if coord.y < rows - 1 {
        neighbors.push(Coord::new(coord.x, coord.y + 1));
    }
    neighbors
}

fn dfs_spanning_tree(coord: Coord, rows: usize, cols: usize, visited: &mut Vec<Vec<bool>>, tree: &mut Vec<(Coord, Coord)>) {
    visited[coord.y][coord.x] = true;
    let mut neighbors = get_neighbors(coord, rows, cols);
    neighbors.shuffle(&mut rand::thread_rng()); // Shuffle the neighbors randomly
    for neighbor in neighbors {
        if !visited[neighbor.y][neighbor.x] {
            tree.push((coord, neighbor));
            dfs_spanning_tree(neighbor, rows, cols, visited, tree);
        }
    }
}

fn create_spanning_tree(rows: usize, cols: usize) -> Vec<(Coord, Coord)> {
    let mut visited = vec![vec![false; cols]; rows];
    let mut tree = Vec::new();
    dfs_spanning_tree(Coord::new(0, 0), rows, cols, &mut visited, &mut tree);
    tree
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    const CELL_SIZE: usize = 10;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let rows = HEIGHT / CELL_SIZE;
    let cols = WIDTH / CELL_SIZE;
    let spanning_tree = create_spanning_tree(rows, cols);

    let mut window = Window::new("Rust Graphics", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));

    let mut drawn = false;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if !drawn{
            drawn = draw_maze(&mut buffer, &spanning_tree, CELL_SIZE, WIDTH, HEIGHT, &mut window);
        }
        window.update();
        //println!("{}", window.is_open());
    }
}


fn draw_maze(buffer: &mut Vec<u32>, spanning_tree: &[(Coord, Coord)], cell_size: usize, width: usize, height: usize, window: &mut Window) -> bool{
    //clear_screen(buffer, 0x00_00_00_00);
    for edge in spanning_tree {
        if !window.is_open() || window.is_key_down(Key::Escape){
            println!("CLOSE");
            return true;
        }
        let x1 = edge.0.x * cell_size;
        let y1 = edge.0.y * cell_size;
        let x2 = edge.1.x * cell_size;
        let y2 = edge.1.y * cell_size;
        draw_line(buffer, width, height, x1, y1, x2, y2, 0xFF_FF_FF_FF);
        window.update_with_buffer(buffer, width, height).unwrap();
    }
    true
}



fn draw_line(buffer: &mut Vec<u32>, width: usize, height: usize, mut x0: usize, mut y0: usize, x1: usize,  y1: usize, color: u32) {
    let dx = (x1 as isize - x0 as isize).abs() as isize;
    let dy = -(y1 as isize - y0 as isize).abs() as isize;
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        if x0 < width && y0 < height {
            buffer[x0 + y0 * width] = color;
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 = (x0 as isize + sx) as usize;
        }
        if e2 <= dx {
            err += dx;
            y0 = (y0 as isize + sy) as usize;
        }
    }
}

/*
fn draw_rect(buffer: &mut Vec<u32>, width: usize, height: usize, x: usize, y: usize, w: usize, h: usize, color: u32) {
    for j in y..(y + h) {
        for i in x..(x + w) {
            if i < width && j < height {
                buffer[i + j * width] = color;
            }
        }
    }
} */

fn clear_screen(buffer: &mut Vec<u32>, color: u32) {
    for pixel in buffer.iter_mut() {
        *pixel = color;
    }
}
