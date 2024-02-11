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

fn map_color(value: usize, max_value: usize, num_colors: usize) -> u32 {
    // Calculate the size of each segment
    let segment_size = max_value / num_colors;

    // Calculate the index of the color
    let color_index = (value / segment_size).min(num_colors - 1);

    // Map the color index to an RGB value
    let r = (color_index * 255 / num_colors) as u32;
    let g = ((num_colors - color_index) * 255 / num_colors) as u32;
    let b = 0;

    // Combine RGB values into a single u32 color
    (r << 16) | (g << 8) | b
}


fn cyclic_gradient(value: usize, max_value: usize, num_colors: usize) -> u32 {
    // Calculate the hue angle
    let hue_step = 360.0 / num_colors as f32;
    let hue = (value as f32 / max_value as f32 * 360.0) % 360.0;

    // Convert HSV to RGB
    let h_prime = hue / 60.0;
    let chroma = 1.0;
    let x = chroma * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let (r, g, b) = match h_prime as i32 {
        0..=1 => (chroma, x, 0.0),
        1..=2 => (x, chroma, 0.0),
        2..=3 => (0.0, chroma, x),
        3..=4 => (0.0, x, chroma),
        4..=5 => (x, 0.0, chroma),
        _ => (chroma, 0.0, x),
    };

    // Convert RGB values to u32 color
    let r = (r * 255.0) as u32;
    let g = (g * 255.0) as u32;
    let b = (b * 255.0) as u32;

    (r << 16) | (g << 8) | b
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
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 800;
    const CELL_SIZE: usize = 10;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let rows = HEIGHT / CELL_SIZE;
    let cols = WIDTH / CELL_SIZE;
    let spanning_tree = create_spanning_tree(rows, cols);

    let mut window = Window::new("Rust Graphics", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));

    let mut drawn = false;
    //println!("{:?}", spanning_tree);
    let mut max_level = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if !drawn{
            //dfs(&mut buffer, &spanning_tree, CELL_SIZE, WIDTH, HEIGHT, &mut window, 0);
            dfs_get_max_level(&spanning_tree, 0, &mut max_level);
            println!("Deepest level: {}", max_level);
            draw_maze(&mut buffer, &spanning_tree, CELL_SIZE, WIDTH, HEIGHT, &mut window);
            dfs(&mut buffer, &spanning_tree, CELL_SIZE, WIDTH, HEIGHT, &mut window, 0, max_level);    
            drawn = true;
        }
        
        window.update();
        //println!("{}", window.is_open());
    }
}

fn draw_maze(buffer: &mut Vec<u32>, spanning_tree: &[(Coord, Coord)], cell_size: usize, width: usize, height: usize, window: &mut Window){
    //clear_screen(buffer, 0x00_00_00_00);
    for edge in spanning_tree {
        if !window.is_open() || window.is_key_down(Key::Escape){
            //println!("CLOSE");
            return;
        }
        let x1 = edge.0.x * cell_size;
        let y1 = edge.0.y * cell_size;
        let x2 = edge.1.x * cell_size;
        let y2 = edge.1.y * cell_size;
        draw_line(buffer, width, height, x1, y1, x2, y2, 0xFF_FF_FF_FF);
        window.update_with_buffer(buffer, width, height).unwrap();
    }
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



fn dfs(buffer: &mut Vec<u32>, spanning_tree: &[(Coord, Coord)], cell_size: usize, width: usize, height: usize, window: &mut Window, level: usize, max_level: usize){
    if let Some(edge) = spanning_tree.get(level) {
        if !window.is_open() || window.is_key_down(Key::Escape){
            return;
        }
        println!("{:?} : level: {}", edge, level);
        // Draw the edge on the buffer
        let x1 = edge.0.x * cell_size;
        let y1 = edge.0.y * cell_size;
        let x2 = edge.1.x * cell_size;
        let y2 = edge.1.y * cell_size;
        draw_line(buffer, width, height, x1, y1, x2, y2, cyclic_gradient(level, max_level, 20));
        
        // Update the window with the buffer
        window.update_with_buffer(buffer, width, height).unwrap();
        
        // Continue DFS to the next level
        dfs(buffer, spanning_tree, cell_size, width, height, window, level + 1, max_level);
    }
}

fn dfs_get_max_level(spanning_tree: &[(Coord, Coord)], level: usize, max_level: &mut usize) {
    if let Some(_edge) = spanning_tree.get(level) {
        // Check if this level is deeper than the current maximum level
        if level > *max_level {
            *max_level = level;
        }
        // Continue DFS to the next level
        dfs_get_max_level(spanning_tree, level + 1, max_level)
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
