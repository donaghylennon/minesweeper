use std::collections::HashSet;
use std::time::{Duration, Instant};

use rand::Rng;
use sdl2::image::InitFlag;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::render::BlendMode;
use sdl2::pixels::Color;

#[derive(Clone, Copy, Debug)]
struct Size(usize, usize);

#[derive(Debug)]
struct Field {
    size: Size,
    bombs: Box<[bool]>,
    clicked: HashSet<(usize, usize)>
}

impl Field {
    fn new() -> Self {
        let size = Size(20, 20);
        let mut bombs: Box<[bool; 400]> = Box::new([false; 400]);
        let mut rng = rand::thread_rng();
        let clicked = HashSet::new();
        bombs.fill_with(|| rng.gen_bool(0.1));
        Field {
            size,
            bombs,
            clicked
        }
    }

    fn bomb_at(&self, x: usize, y: usize) -> bool {
        self.bombs[y*self.size.0 + x]
    }

    fn num_surrounding_bombs(&self, x: usize, y: usize) -> u32 {
        let mut count = 0;
        for pos in self.surrounding_positions(x, y) {
            if self.bombs[pos.1*self.size.0 + pos.0] {
                count += 1;
            }
        }
        count
    }

    fn surrounding_positions(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for i in 0..=2 {
            for j in 0..=2 {
                if i == 1 && j == 1 { continue; }
                let xpos = (x+i) as isize -1;
                let ypos = (y+j) as isize -1;
                if xpos >= 0 && ypos >= 0 &&
                    xpos < self.size.0 as isize && ypos < self.size.1 as isize {
                        positions.push((xpos as usize, ypos as usize));
                }
            }
        }
        positions
    }

    fn clicked_at(&self, x: usize, y: usize) -> bool {
        self.clicked.contains(&(x,y))
    }

    fn click(&mut self, x: usize, y: usize) {
        self.clicked.insert((x,y));
        if !self.bomb_at(x, y) && self.num_surrounding_bombs(x, y) == 0 {
            for pos in self.surrounding_positions(x, y) {
                if !self.clicked.contains(&pos) {
                    self.click(pos.0, pos.1);
                }
            }
        }
    }

    fn has_won(&mut self) -> bool {
        let mut won = true;
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                if !self.clicked.contains(&(i, j)) {
                    won = false;
                }
            }
        }
        won
    }
}

fn main() {
    let mut field = Field::new();
    let fps = 60.0;

    let sdl = sdl2::init().expect("Failed to init SDL");
    let video = sdl.video().expect("Failed to init video subsystem");
    //let sdl_img = sdl2::image::init(InitFlag::PNG);
    let mut event_pump = sdl.event_pump().expect("Failed to retrieve event pump");
    let window = video.window("Minesweeper", 20*32, 20*32)
        .position_centered()
        .build()
        .expect("Failed to build window");

    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Failed to build canvas");
    canvas.set_blend_mode(BlendMode::Blend);
    canvas.set_draw_color(Color::RGBA(255,255,255,100));

    let tc = canvas.texture_creator();
    let t_square = tc.load_texture("res/Square.png").expect("Failed to load texture");
    let t_mine = tc.load_texture("res/Mine.png").expect("Failed to load texture");
    let t_empty = tc.load_texture("res/Empty.png").expect("Failed to load texture");
    let t_one = tc.load_texture("res/One.png").expect("Failed to load texture");
    let t_two = tc.load_texture("res/Two.png").expect("Failed to load texture");
    let t_three = tc.load_texture("res/Three.png").expect("Failed to load texture");
    let t_four = tc.load_texture("res/Four.png").expect("Failed to load texture");
    let t_five = tc.load_texture("res/Five.png").expect("Failed to load texture");
    let t_six = tc.load_texture("res/Six.png").expect("Failed to load texture");
    let t_seven = tc.load_texture("res/Seven.png").expect("Failed to load texture");
    let t_eight = tc.load_texture("res/Eight.png").expect("Failed to load texture");
    let mut cursor_rect = Rect::new(0, 0, 32, 32);

    let mut prev_time = Instant::now();
    let frame_duration = Duration::from_secs_f32(1.0/fps);

    let mut quit = false;
    let mut finished = false;
    let mut mousedown = false;
    let mut mousepos = (0,0);
    while !quit {
        let current_time = Instant::now();
        if current_time - prev_time >= frame_duration {
            prev_time = current_time;
            if !finished && mousedown {
                field.click(mousepos.0, mousepos.1);
                if field.bomb_at(mousepos.0, mousepos.1) {
                    finished = true;
                }
                mousedown = false;
            }
            canvas.clear();

            for i in 0..field.size.0 {
                for j in 0..field.size.1 {
                    if field.clicked_at(i, j) {
                        if field.bomb_at(i, j) {
                            canvas.copy(&t_mine, None, Rect::new(32*i as i32, 32*j as i32, 32, 32))
                                .expect("Failed to copy to canvas");
                        } else {
                            match field.num_surrounding_bombs(i, j) {
                                1 => canvas.copy(&t_one, None, Rect::new(32*i as i32, 32*j as i32, 32, 32))
                                    .expect("Failed to copy to canvas"),
                                2 => canvas.copy(&t_two, None, Rect::new(32*i as i32, 32*j as i32, 32, 32))
                                    .expect("Failed to copy to canvas"),
                                3 => canvas.copy(&t_three, None, Rect::new(32*i as i32, 32*j as i32, 32, 32))
                                    .expect("Failed to copy to canvas"),
                                4 => canvas.copy(&t_four, None, Rect::new(32*i as i32, 32*j as i32, 32, 32))
                                    .expect("Failed to copy to canvas"),
                                5 => canvas.copy(&t_five, None, Rect::new(32*i as i32, 32*j as i32, 32, 32))
                                    .expect("Failed to copy to canvas"),
                                6 => canvas.copy(&t_six, None, Rect::new(32*i as i32, 32*j as i32, 32, 32))
                                    .expect("Failed to copy to canvas"),
                                7 => canvas.copy(&t_seven, None, Rect::new(32*i as i32, 32*j as i32, 32, 32))
                                    .expect("Failed to copy to canvas"),
                                8 => canvas.copy(&t_eight, None, Rect::new(32*i as i32, 32*j as i32, 32, 32))
                                    .expect("Failed to copy to canvas"),
                                _ => canvas.copy(&t_empty, None, Rect::new(32*i as i32, 32*j as i32, 32, 32))
                                    .expect("Failed to copy to canvas")
                            }
                        }
                    } else {
                        canvas.copy(&t_square, None, Rect::new(32*i as i32, 32*j as i32, 32, 32))
                            .expect("Failed to copy to canvas");
                    }
                }
            }
            if !finished {
                cursor_rect.reposition((mousepos.0 as i32*32, mousepos.1 as i32*32));
                canvas.fill_rect(cursor_rect).expect("Failed to draw cursor");
            }

            canvas.present();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit{..} => quit = true,
                    Event::MouseButtonDown {
                        timestamp:_,
                        window_id:_,
                        which:_,
                        mouse_btn:_,
                        clicks:_,
                        x, y
                    } => {
                        mousepos = (x as usize/32, y as usize/32);
                        mousedown = true;
                    }
                    Event::MouseMotion {
                        timestamp:_,
                        window_id:_,
                        which:_,
                        mousestate:_,
                        x, y,
                        xrel:_, yrel:_
                    } => mousepos = (x as usize/32, y as usize/32),
                    _ => ()
                }
            }
        }
    }
}
