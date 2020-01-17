use std::collections::LinkedList;
use ggez::{graphics, Context, GameResult};

use crate::components::{Segment, Direction, GridPosition, Food};

#[derive(Clone, Copy, Debug)]
pub enum Ate {
    Itself,
    Food,
}

pub struct Snake {
  pub head: Segment,
  pub dir: Direction,
  pub body: LinkedList<Segment>,
  pub ate: Option<Ate>,
  pub last_update_dir: Direction,
  pub next_dir: Option<Direction>,
}

impl Snake {
  pub fn new(pos: GridPosition) -> Self {
      let mut body = LinkedList::new();
      body.push_back(Segment::new((pos.x - 1, pos.y).into()));
      Snake {
          head: Segment::new(pos),
          dir: Direction::Right,
          last_update_dir: Direction::Right,
          body: body,
          ate: None,
          next_dir: None
      }
  }

  /// A helper function that determines whether
  /// the snake eats a given piece of Food based
  /// on its current position
  pub fn eats(&self, food: &Food) -> bool {
      if self.head.pos == food.pos {
          true
      } else {
          false
      }
  }

  /// A helper function that determines whether
  /// the snake eats itself based on its current position
  pub fn eats_self(&self) -> bool {
      for seg in self.body.iter() {
          if self.head.pos == seg.pos {
              return true;
          }
      }
      false
  }

  /// The main update function for our snake which gets called every time
  /// we want to update the game state.
  pub fn update(&mut self, food: &Food) {
      // If `last_update_dir` has already been updated to be the same as `dir`
      // and we have a `next_dir`, then set `dir` to `next_dir` and unset `next_dir`
      if self.last_update_dir == self.dir && self.next_dir.is_some() {
          self.dir = self.next_dir.unwrap();
          self.next_dir = None;
      }
      // First we get a new head position by using our `new_from_move` helper
      // function from earlier. We move our head in the direction we are currently
      // heading.
      let new_head_pos = GridPosition::new_from_move(self.head.pos, self.dir);
      // Next we create a new segment will be our new head segment using the
      // new position we just made.
      let new_head = Segment::new(new_head_pos);
      // Then we push our current head Segment onto the front of our body
      self.body.push_front(self.head);
      // And finally make our actual head the new Segment we created. This has
      // effectively moved the snake in the current direction.
      self.head = new_head;
      // Next we check whether the snake eats itself or some food, and if so,
      // we set our `ate` member to reflect that state.
      if self.eats_self() {
          self.ate = Some(Ate::Itself);
      } else if self.eats(food) {
          self.ate = Some(Ate::Food);
      } else {
          self.ate = None
      }
      // If we didn't eat anything this turn, we remove the last segment from our body,
      // which gives the illusion that the snake is moving. In reality, all the segments stay
      // stationary, we just add a segment to the front and remove one from the back. If we eat
      // a piece of food, then we leave the last segment so that we extend our body by one.
      if let None = self.ate {
          self.body.pop_back();
      }
      // And set our last_update_dir to the direction we just moved.
      self.last_update_dir = self.dir;
  }

  /// Here we have the Snake draw itself. This is very similar to how we saw the Food
  /// draw itself earlier.
  ///
  /// Again, note that this approach to drawing is fine for the limited scope of this
  /// example, but larger scale games will likely need a more optimized render path
  /// using SpriteBatch or something similar that batches draw calls.
  pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
      // We first iterate through the body segments and draw them.
      for seg in self.body.iter() {
          // Again we set the color (in this case an orangey color)
          // and then draw the Rect that we convert that Segment's position into
          let rectangle = graphics::Mesh::new_rectangle(
              ctx,
              graphics::DrawMode::fill(),
              seg.pos.into(),
              [0.3, 0.3, 0.0, 1.0].into(),
          )?;
          graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
      }
      // And then we do the same for the head, instead making it fully red to distinguish it.
      let rectangle = graphics::Mesh::new_rectangle(
          ctx,
          graphics::DrawMode::fill(),
          self.head.pos.into(),
          [1.0, 0.5, 0.0, 1.0].into(),
      )?;
      graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
      Ok(())
  }
}