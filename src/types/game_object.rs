pub trait GameObject {
    fn update(&mut self);
    fn draw(&self);
    fn reset_position(&mut self);
}
