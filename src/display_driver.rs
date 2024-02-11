extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::TextureQuery;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{EventPump, Sdl};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct DisplayDriver {
    pub ctx: Sdl,
    canvas: WindowCanvas,
    ttf_context: Sdl2TtfContext,
    scale: usize,
}

impl DisplayDriver {
    pub fn build(scale: usize) -> DisplayDriver {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

        let window = video_subsystem
            .window("Game of Life", WIDTH, HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        DisplayDriver {
            ctx: sdl_context,
            canvas,
            ttf_context,
            scale,
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        self.canvas.output_size().unwrap()
    }

    fn draw_board(
        &mut self,
        board: &mut Vec<Vec<Pixel>>,
        on_color: Color,
        off_color: Color,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        board.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|pixel| {
                let color = if pixel.on { on_color } else { off_color };
                self.canvas.set_draw_color(color);
                let (x, y) = pixel.get_coords();

                let _ = self.canvas.fill_rect(Rect::new(
                    (x * self.scale) as i32,
                    (y * self.scale) as i32,
                    self.scale as u32,
                    self.scale as u32,
                ));
            });
        });

        self.canvas.set_draw_color(Color::GRAY);
        let x_squares_per_row = board[0].len();
        let y_squares_per_row = board.len();

        // Vertical lines
        for x in 0..=x_squares_per_row {
            let x_pos = x * self.scale;
            self.canvas.draw_line(
                Point::new(x_pos as i32, 0),
                Point::new(x_pos as i32, HEIGHT as i32),
            )?;
        }

        // Horizontal lines
        for y in 0..=y_squares_per_row {
            let y_pos = y * self.scale;
            self.canvas.draw_line(
                Point::new(0, y_pos as i32),
                Point::new(WIDTH as i32, y_pos as i32),
            )?;
        }
        self.canvas.present();
        Ok(())
    }

    pub fn draw(&mut self, board: &mut Vec<Vec<Pixel>>, paused: bool) -> Result<(), String> {
        let on_color = if !paused { Color::WHITE } else { Color::RED };
        let off_color = if !paused { Color::BLACK } else { Color::BLACK };
        self.draw_board(board, on_color, off_color);
        /* self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        board.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|pixel| {
                let color = if pixel.on { Color::WHITE } else { Color::BLACK };
                self.canvas.set_draw_color(color);
                let (x, y) = pixel.get_coords();

                let _ = self.canvas.fill_rect(Rect::new(
                    (x * self.scale) as i32,
                    (y * self.scale) as i32,
                    self.scale as u32,
                    self.scale as u32,
                ));
            });
        });

        self.canvas.set_draw_color(Color::GRAY);
        let x_squares_per_row = board[0].len();
        let y_squares_per_row = board.len();

        // Vertical lines
        for x in 0..=x_squares_per_row {
            let x_pos = x * self.scale;
            self.canvas.draw_line(
                Point::new(x_pos as i32, 0),
                Point::new(x_pos as i32, HEIGHT as i32),
            )?;
        }

        // Horizontal lines
        for y in 0..=y_squares_per_row {
            let y_pos = y * self.scale;
            self.canvas.draw_line(
                Point::new(0, y_pos as i32),
                Point::new(WIDTH as i32, y_pos as i32),
            )?;
        }
        self.canvas.present();*/
        Ok(())
    }

    pub fn paused(&mut self, board: &mut Vec<Vec<Pixel>>) -> Result<(), String> {
        self.draw_board(board, Color::RED, Color::BLACK);
        /*let texture_creator = self.canvas.texture_creator();

        // Load a font
        let mut font = self
            .ttf_context
            .load_font("fonts/GeistMonoNerdFont-Regular.otf", 64)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render("PAUSED")
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();
        println!("w: {width}, h: {height}");

        // If the example text is too big for the screen, downself.scale it (and center irregardless)
        let cx = (WIDTH - width) / 2;
        let cy = (HEIGHT - height) / 2;
        let text_rect = Rect::new(cx as i32, cy as i32, width, height);

        self.canvas.copy(&texture, None, Some(text_rect))?;
        self.canvas.present();
        Ok(())*/
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    x: usize,
    y: usize,
    on: bool,
    alive_next_generation: bool,
}

impl Pixel {
    pub fn new(x: usize, y: usize, on: bool) -> Pixel {
        Pixel {
            x,
            y,
            on,
            alive_next_generation: false,
        }
    }

    pub fn turn_on(&mut self) {
        self.on = true;
    }

    pub fn turn_off(&mut self) {
        self.on = false;
    }

    pub fn survive(&mut self) {
        self.alive_next_generation = true;
    }

    pub fn kill(&mut self) {
        self.alive_next_generation = false;
    }

    pub fn is_alive_next_generation(&self) -> bool {
        self.alive_next_generation
    }

    pub fn is_on(&self) -> bool {
        self.on
    }

    pub fn get_coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_init() {
        let pixel = Pixel::new(5, 10, false);
        assert_eq!(5, pixel.x);
        assert_eq!(10, pixel.y);
        assert_eq!(false, pixel.on);
    }

    #[test]
    fn turn_off_pixel() {
        let mut p = Pixel::new(5, 10, true);
        assert!(p.on);
        p.turn_off();
        assert!(!p.on);
    }

    #[test]
    fn turn_on_pixel() {
        let mut p = Pixel::new(5, 10, false);
        assert!(!p.on);
        p.turn_on();
        assert!(p.on);
    }
}