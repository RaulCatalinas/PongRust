pub trait GameObject {
    fn update(&mut self);
    fn draw(&self);
    fn set_velocity_x(&mut self, new_velocity_x: f32);
    fn set_velocity_y(&mut self, new_velocity_y: f32);
    fn reset_position(&mut self);
}
