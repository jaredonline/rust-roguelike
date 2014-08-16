use game::Game;
use rendering::TcodRenderingComponent;

pub trait Updates {
    fn update(&mut self, Game);
    fn render(&self, &mut TcodRenderingComponent);
}
