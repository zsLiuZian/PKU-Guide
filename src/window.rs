use minifb::{Window, WindowOptions, MouseMode, MouseButton};
use std::thread;
use std::time::Duration;
use crate::Loc;

pub const WIDTH: usize = 1000;
pub const HEIGHT: usize = 750;

struct MyWindow {
    window: Window,
    buffer: Vec<u32>
}

impl MyWindow {

    fn new() -> Self {

        let buffer: Vec<u32> = {

            let img = image::open("img/map.png").expect("地图加载失败").to_rgb8();

            img
                .pixels()
                .map(|p| {
                    let [r, g, b] = p.0;
                    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
                })
                .collect()
        };

        let mut window = {
            let options = WindowOptions {
                topmost: true,
                resize: false,     
                borderless: false, 
                ..WindowOptions::default()
            };
            Window::new(
                "PKU-Guide",
                WIDTH,
                HEIGHT,
                options,
            )
            .unwrap()
        };

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        Self {
            window,
            buffer,
        }
    }

    fn is_open(&self) -> bool {
        self.window.is_open()
    }

    fn draw_dot(&mut self, x: usize, y: usize, s: usize) {
        
        let point_color = 0xFF007AFF;

        for i in (y - s)..(y + s) {
            for j in (x - s)..(x + s) {
                let dis = i.abs_diff(y).pow(2) + j.abs_diff(x).pow(2);
                if  dis <= s.pow(2) {
                    self.buffer[i * WIDTH + j] = point_color;
                }
            }
        }
    }

    fn draw_line(&mut self, from: Loc, to: Loc) {

        let n_steps = 200;

        let x0 = from.0 as f64;
        let y0 = from.1 as f64;
        let x1 = to.0 as f64;
        let y1 = to.1 as f64;

        let step_x = (x1 - x0) / n_steps as f64;
        let step_y = (y1 - y0) / n_steps as f64;

        let mut sx = x0;
        let mut sy = y0;

        for _ in 0..n_steps {
            sx += step_x;
            sy += step_y;
            self.draw_dot(sx as usize, sy as usize, 3);
            self.update();
            if !self.window.is_open() {
                break;
            }
        }               
    }

    fn update(&mut self) {
        self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();
    }
}

pub fn fetch_loc() -> Loc {

    let mut wd = MyWindow::new();

    let mut x: usize = 0;
    let mut y: usize = 0;
    while wd.is_open() {

        wd.update();

        let mouse_down = wd.window.get_mouse_down(MouseButton::Left);
        if mouse_down {
            if let Some((mx, my)) = wd.window.get_mouse_pos(MouseMode::Discard) {
                x = mx as usize;
                y = my as usize;

                wd.draw_dot(x, y, 6);
                wd.update();
                thread::sleep(Duration::from_secs(1));
                break;
            }
        }
    }
    (x, y)
}

pub fn show_path(v: &Vec<Loc>) {

    let mut wd = MyWindow::new();
    while wd.is_open() {

        for i in 0..v.len() - 1 {
                
            wd.draw_line(v[i], v[i + 1]);

            if !wd.is_open() {
                break;
            }
        }
    }
}