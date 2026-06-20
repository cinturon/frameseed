use serde::{Deserialize, Serialize};
use crate::{Frame, RenderContext, Scene, seeded_rng, Rgba};
use rand::Rng;

pub struct ConwayScene {
    pub cell_size: u32,
    pub density: f32,
}

impl ConwayScene {
    pub fn new(cell_size: u32, density: f32) -> Self {
        Self { cell_size, density }
    }
}

impl Scene for ConwayScene {
    fn name(&self) -> &str {
        "conway"
    }

    fn render(&self, frame: &mut Frame, context: &RenderContext) {
        let (cols, rows) = grid_dims(frame.width, frame.height, self.cell_size);
        let mut grid = seed_grid(cols, rows, context.seed, self.density);
        simulate_grid(&mut grid, context.frame_index);
        render_grid(&grid, frame, self.cell_size);
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ConwayParams {
    #[serde(default = "default_cell_size")]
    pub cell_size: u32,
    #[serde(default = "default_density")]
    pub density: f32,
}

impl Default for ConwayParams {
    fn default() -> Self {
        Self { cell_size: 4, density: 0.3 }
    }
}

impl ConwayParams {
    pub fn new(cell_size: u32, density: f32) -> Self {
        Self { cell_size, density }
    }
}

fn default_cell_size() -> u32 {
    4
}

fn default_density() -> f32 {
    0.3
}

fn grid_dims(frame_width: u32, frame_height: u32, cell_size: u32) -> (u32, u32) {
    let grid_width = (frame_width + cell_size - 1) / cell_size;
    let grid_height = (frame_height + cell_size - 1) / cell_size;
    (grid_width, grid_height)
}

fn seed_grid(columns: u32, rows: u32, seed: u64, density: f32) -> Vec<Vec<bool>> {
    let mut rng = seeded_rng(seed);
    
    let mut grid = vec![vec![false; columns as usize]; rows as usize];

    for row in &mut grid {
        for cell in row {
            if rng.random::<f32>() < density {
                *cell = true;
            }
        }
    }
    grid
}

fn count_neighbors(grid: &[Vec<bool>], x: usize, y: usize) -> u8 {
    let cols = grid[0].len();
    let rows = grid.len();  
    let mut count = 0;
    
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= cols as i32 || ny < 0 || ny >= rows as i32 {
                continue;
            }
            if grid[ny as usize][nx as usize] {
                count += 1;
            }
        }
    }
    count
}

fn step_grid(grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = grid.clone();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let live = grid[y][x];
            let neighbors = count_neighbors(grid, x, y);

            new_grid[y][x] = if live{
                neighbors == 2 || neighbors == 3
            } else {
                neighbors == 3
            }
        }
    }
    *grid = new_grid;
}

fn simulate_grid(grid: &mut Vec<Vec<bool>>, generations: u32) {
    for _ in 0..generations {
        step_grid(grid);
    }
}

fn render_grid(grid: &[Vec<bool>], frame: &mut Frame, cell_size: u32) {
    for (gy, row) in grid.iter().enumerate() {
        for (gx, &alive) in row.iter().enumerate() {
            if !alive {
                continue;
            }

            let x = gx as u32 * cell_size;
            let y = gy as u32 * cell_size;
              
            for dy in 0..cell_size {
                for dx in 0..cell_size {
                    frame.set_pixel(x + dx, y + dy, Rgba::white());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_conway_frame(
        width: u32,
        height: u32,
        frame_index: u32,
        seed: u64,
        cell_size: u32,
        density: f32,
    ) -> Frame {
        let mut frame = Frame::new(width, height);
        let ctx = RenderContext::new(frame_index, 120, 24.0, seed);
        let scene = ConwayScene::new(cell_size, density);
        scene.render(&mut frame, &ctx);
        frame
    }

    fn blinker_horizontal() -> Vec<Vec<bool>> {
        vec![
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
        ]
    }

    #[test]
    fn test_grid_dims() {
        assert_eq!(grid_dims(100, 100, 4), (25, 25));
        assert_eq!(grid_dims(101, 65, 4), (26, 17));
    }

    #[test]
    fn test_seed_grid() {
        let grid = seed_grid(10, 10, 42, 0.3);
        assert_eq!(grid.len(), 10);
        assert_eq!(grid[0].len(), 10);
        assert!(grid.iter().flatten().any(|&cell| cell));
    }

    #[test]
    fn seed_grid_is_deterministic_for_same_seed() {
        let a = seed_grid(10, 10, 42, 0.3);
        let b = seed_grid(10, 10, 42, 0.3);
        assert_eq!(a, b);
    }

    #[test]
    fn test_count_neighbors() {
        let grid = vec![
            vec![true, false, true],
            vec![false, true, false],
            vec![true, false, true],
        ];
        assert_eq!(count_neighbors(&grid, 1, 1), 4);
    }

    #[test]
    fn blinker_oscillates_horizontally_and_vertically() {
        let mut grid = blinker_horizontal();

        step_grid(&mut grid);
        assert!(!grid[1][1] && grid[0][2] && grid[1][2] && grid[2][2]);

        step_grid(&mut grid);
        assert!(grid[1][1] && grid[1][2] && grid[1][3]);
        assert!(!grid[0][2] && !grid[2][2]);
    }

    #[test]
    fn test_frame0_is_all_black_with_zero_density() {
        let frame = render_conway_frame(16, 16, 0, 42, 4, 0.0);
        assert_eq!(frame.get_pixel(0, 0), Some(Rgba::black()));
        assert_eq!(frame.get_pixel(8, 8), Some(Rgba::black()));
    }

    #[test]
    fn same_seed_and_frame_produce_identical_pixels() {
        let a = render_conway_frame(64, 64, 10, 42, 4, 0.3);
        let b = render_conway_frame(64, 64, 10, 42, 4, 0.3);
        assert_eq!(a.get_pixel(16, 16), b.get_pixel(16, 16));
    }

    #[test]
    fn later_frames_differ_from_frame_zero() {
        let frame0 = render_conway_frame(64, 64, 0, 42, 4, 0.3);
        let frame10 = render_conway_frame(64, 64, 10, 42, 4, 0.3);
        assert_ne!(frame0.get_pixel(16, 16), frame10.get_pixel(16, 16));
    }
}