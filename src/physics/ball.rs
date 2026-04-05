use crate::entities::{ball::Ball, paddle::Paddle};

pub fn check_ball_paddle_collision(ball: &Ball, paddle: &Paddle) -> bool {
    ball.x() + ball.radius() >= paddle.x()
        && ball.x() - ball.radius() <= paddle.x() + paddle.width()
        && ball.y() + ball.radius() >= paddle.y()
        && ball.y() - ball.radius() <= paddle.y() + paddle.height()
}

pub fn check_ball_wall_collision(
    ball: &Ball,
    screen_width: f32,
    screen_height: f32,
) -> (bool, bool, bool) {
    let collision_left_wall = ball.x() - ball.radius() <= 0.0;
    let collision_right_wall = ball.x() + ball.radius() >= screen_width;
    let collision_y = ball.y() - ball.radius() <= 0.0 || ball.y() + ball.radius() >= screen_height;

    (collision_left_wall, collision_right_wall, collision_y)
}
