use std::collections::VecDeque;
use macroquad::{color::BLACK, time::get_time, window::{Conf, clear_background, next_frame}};
use tetris::{agent::agent::TetrisAgent, engine::{action::Action, core::TetrisEngine}, pieces::get_pieces, render::TetrisRenderer};
use ::rand::rng;

const WIDTH: i32 = 440;
const HEIGHT: i32 = 570;

const LATTICE_WIDTH: usize = 10;
const LATTICE_HEIGHT: usize = 20;

const ACTION_COOLDOWN: f64 = 0.03;


#[macroquad::main(window_conf)]
async fn main() {
    let renderer = TetrisRenderer::new(30.0, 30.0, 25.0).await;

    let pieces = get_pieces(LATTICE_WIDTH);

    let agent = TetrisAgent {
        holes_w: -1.5,
        height_w: -2.0,
        bumpiness_w: -1.0,
        lookahead_depth: 1,
    };
    let mut action_queue: VecDeque<Action> = VecDeque::new();

    loop {
        let mut engine = TetrisEngine::new((LATTICE_WIDTH, LATTICE_HEIGHT), &pieces, rng());

        let mut last_update = get_time();
        let mut last_action = get_time();

        while !engine.game_over {
            clear_background(BLACK);

            // Compute next actions if action queue is empty
            if action_queue.is_empty() {
                action_queue.extend(agent.get_action(&engine));
            } else if get_time() - last_action > ACTION_COOLDOWN { /* process action with a cooldown */
                if let Some(action) = action_queue.pop_front() {
                    let _ = engine.step(action);
                }
                last_action = get_time();
            }

            if get_time() - last_update > 0.5 {
                let _ = engine.step(Action::MoveDown);
                last_update = get_time();
            }

            renderer.draw(&engine);
            next_frame().await;
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "TETRIS".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        ..Default::default()
    }
}
