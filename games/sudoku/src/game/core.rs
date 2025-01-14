use ggez::{event, graphics, mint::Point2, Context, GameResult};

// use crate::game::entity::*;
use crate::game::{
    constants::*,
    context,
    game_states::{
        leader_board::LeaderBoard, main_menu::MainMenu, playing::Playing,
        select_difficulty::SelectDifficulty, *,
    },
};

pub fn run() {
    let (ctx, events_loop) = ggez::ContextBuilder::new("Sudoku", "alimulap")
        .window_setup(ggez::conf::WindowSetup::default().title("Sudoku"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .unwrap();

    let state = App::new(&ctx, GameState::MainMenu);
    event::run(ctx, events_loop, state);
}

struct App {
    current_state: Box<dyn StateTrait>,
    addon_ctx: context::AddOnContext,
}

impl App {
    fn new(ctx: &Context, initial_state: GameState) -> Self {
        let current_state: Box<dyn StateTrait> = match initial_state {
            GameState::MainMenu => Box::new(MainMenu::new(ctx)),
            GameState::SelectDifficulty => Box::new(SelectDifficulty::new(ctx)),
            GameState::Playing => Box::new(Playing::new(ctx, &context::AddOnContext::new_forced())),
            GameState::LeaderBoard => Box::new(LeaderBoard::new(ctx)),
        };
        App {
            current_state,
            addon_ctx: context::AddOnContext::new(),
        }
    }

    fn change_state(&mut self, ctx: &Context, new_state: GameState) {
        let new_state: Box<dyn StateTrait> = match new_state {
            GameState::MainMenu => Box::new(MainMenu::new(ctx)),
            GameState::SelectDifficulty => Box::new(SelectDifficulty::new(ctx)),
            GameState::Playing => Box::new(Playing::new(ctx, &self.addon_ctx)),
            GameState::LeaderBoard => Box::new(LeaderBoard::new(ctx)),
        };
        let old_state = std::mem::replace(&mut self.current_state, new_state);
        std::mem::drop(old_state);
    }
}

impl event::EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(new_state) = self.current_state.update(ctx, &mut self.addon_ctx)? {
            self.change_state(ctx, new_state);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));
        self.current_state.draw(ctx, &mut canvas)?;
        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.current_state
            .mouse_button_down_event(ctx, &button, &Point2 { x, y })?;
        Ok(())
    }
}
