use std::collections::BTreeMap;

use ggez::{
    glam::Vec2,
    graphics::{self, Color, Rect, Text},
    Context, GameResult,
};

use crate::game::{constants::*, game_states::*, ui::*};

pub struct MainMenu {
    texts: BTreeMap<&'static str, Text>,
    buttons: BTreeMap<&'static str, Button>,
    background: graphics::Mesh,
    change_state: Option<GameState>,
}

impl MainMenu {
    pub fn new(ctx: &Context) -> Self {
        let mut texts = BTreeMap::new();
        texts.insert(
            "0_Title",
            Text::new(
                graphics::TextFragment::new("SUDOKU")
                    .color(Color::WHITE)
                    .scale(80.),
            )
            .set_layout(graphics::TextLayout::center())
            .to_owned(),
        );
        texts.insert(
            "1_Author",
            Text::new(
                graphics::TextFragment::new("Made by alimulap")
                    .color(Color::WHITE)
                    .scale(15.),
            )
            .set_layout(graphics::TextLayout::center())
            .to_owned(),
        );
        let mut buttons = BTreeMap::new();
        buttons.insert(
            "Play",
            Button::new(
                ctx,
                Rect::new(320., 200., 80., 40.),
                Text::new(
                    graphics::TextFragment::new("PLAY")
                        .color(Color::WHITE)
                        .scale(20.),
                )
                .set_layout(graphics::TextLayout::center())
                .to_owned(),
            ),
        );
        buttons.insert(
            "LeaderBoard",
            Button::new(
                ctx,
                Rect::new(280., 260., 160., 40.),
                Text::new(
                    graphics::TextFragment::new("LEADERBOARD")
                        .color(Color::WHITE)
                        .scale(20.),
                )
                .set_layout(graphics::TextLayout::center())
                .to_owned(),
            ),
        );
        buttons.insert(
            "Exit",
            Button::new(
                ctx,
                Rect::new(320., 320., 80., 40.),
                Text::new(
                    graphics::TextFragment::new("EXIT")
                        .color(Color::WHITE)
                        .scale(20.),
                )
                .set_layout(graphics::TextLayout::center())
                .to_owned(),
            ),
        );
        let vertices = [
            graphics::Vertex {
                position: [0., 0.],
                uv: [0., 0.],
                color: [0.001, 0., 0.001, 1.],
            },
            graphics::Vertex {
                position: [SCREEN_SIZE.0, 0.],
                uv: [SCREEN_SIZE.0, 0.],
                color: [0., 0., 0.01, 1.],
            },
            graphics::Vertex {
                position: [SCREEN_SIZE.0 / 2., SCREEN_SIZE.1 / 2.],
                uv: [SCREEN_SIZE.0 / 2., SCREEN_SIZE.1 / 2.],
                color: [0.015, 0., 0.02, 1.],
            },
            graphics::Vertex {
                position: [SCREEN_SIZE.0, SCREEN_SIZE.1],
                uv: [SCREEN_SIZE.0, SCREEN_SIZE.1],
                color: [0.001, 0., 0.001, 1.],
            },
            graphics::Vertex {
                position: [0., SCREEN_SIZE.1],
                uv: [0., SCREEN_SIZE.1],
                color: [0., 0., 0.01, 1.],
            },
        ];
        let indices = [0, 1, 2, 2, 1, 3, 3, 2, 4, 4, 2, 0];
        let background = graphics::Mesh::from_data(
            ctx,
            graphics::MeshData {
                vertices: &vertices,
                indices: &indices,
            },
        );
        MainMenu {
            texts,
            buttons,
            background,
            change_state: None,
        }
    }
}

impl StateTrait for MainMenu {
    fn update(
        &mut self,
        _ctx: &Context,
        _addon_ctx: &mut AddOnContext,
    ) -> GameResult<Option<GameState>> {
        if let Some(new_state) = self.change_state {
            self.change_state = None;
            return Ok(Some(new_state));
        }
        Ok(None)
    }

    fn draw(&mut self, _ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        canvas.draw(&self.background, graphics::DrawParam::default());

        for (_key, button) in self.buttons.iter_mut() {
            button.draw(canvas);
        }

        for (key, text) in self.texts.iter() {
            match *key {
                "0_Title" => canvas.draw(text, Vec2::new(360., 100.)),
                "1_Author" => canvas.draw(text, Vec2::new(640., 450.)),
                _ => (),
            }
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: &MouseButton,
        point: &Point2<f32>,
    ) -> GameResult {
        for (key, buttonui) in self.buttons.iter_mut() {
            if buttonui.rect.contains(*point) && *button == MouseButton::Left {
                match *key {
                    "Play" => self.change_state = Some(GameState::SelectDifficulty),
                    "LeaderBoard" => self.change_state = Some(GameState::LeaderBoard),
                    "Exit" => ctx.request_quit(),
                    _ => (),
                }
            }
        }
        Ok(())
    }
}
