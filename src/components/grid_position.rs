use ggez::graphics;
use rand::Rng;

use crate::components::Direction;

const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

trait ModuloSigned {
  fn modulo(&self, n: Self) -> Self;
}

impl<T> ModuloSigned for T
where
  T: std::ops::Add<Output = T> + std::ops::Rem<Output = T> + Clone,
{
  fn modulo(&self, n: T) -> T {
      // Because of our trait bounds, we can now apply these operators.
      (self.clone() % n.clone() + n.clone()) % n.clone()
  }
}

impl GridPosition {
  pub fn new(x: i16, y: i16) -> Self {
      GridPosition { x, y }
  }

  pub fn random(max_x: i16, max_y: i16) -> Self {
      let mut rng = rand::thread_rng();
      // We can use `.into()` to convert from `(i16, i16)` to a `GridPosition` since
      // we implement `From<(i16, i16)>` for `GridPosition` below.
      (
          rng.gen_range::<i16, i16, i16>(0, max_x),
          rng.gen_range::<i16, i16, i16>(0, max_y),
      )
          .into()
  }

  pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
      match dir {
          Direction::Up => GridPosition::new(pos.x, (pos.y - 1).modulo(GRID_SIZE.1)),
          Direction::Down => GridPosition::new(pos.x, (pos.y + 1).modulo(GRID_SIZE.1)),
          Direction::Left => GridPosition::new((pos.x - 1).modulo(GRID_SIZE.0), pos.y),
          Direction::Right => GridPosition::new((pos.x + 1).modulo(GRID_SIZE.0), pos.y),
      }
  }
}

impl From<GridPosition> for graphics::Rect {
  fn from(pos: GridPosition) -> Self {
      graphics::Rect::new_i32(
          pos.x as i32 * GRID_CELL_SIZE.0 as i32,
          pos.y as i32 * GRID_CELL_SIZE.1 as i32,
          GRID_CELL_SIZE.0 as i32,
          GRID_CELL_SIZE.1 as i32,
      )
  }
}

impl From<(i16, i16)> for GridPosition {
  fn from(pos: (i16, i16)) -> Self {
      GridPosition { x: pos.0, y: pos.1 }
  }
}