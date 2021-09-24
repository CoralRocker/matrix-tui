#![allow(dead_code, unused_imports, non_snake_case)]

use tui::buffer::*;
use tui::widgets::*;
use tui::style::*;
use tui::layout::*;
use rand::prelude::*;

const MIN_STREAM_LENGTH: u8 = 16;
const MAX_STREAM_LENGTH: u8 = 64;

#[derive(Clone)]
struct MatrixStream {
    chars: Vec<char>,
    pos: (usize, usize),
}

impl MatrixStream {
    fn new(_pos: usize) -> MatrixStream {
        let mut cvec = Vec::new();
        let mut rng = thread_rng();
        for i in 0..rng.gen_range(MIN_STREAM_LENGTH..MAX_STREAM_LENGTH) {
            if i == 0 {
                cvec.push(rng.gen_range(33..127) as u8 as char);
            }else{
                cvec.push(0x00 as char);
            }
        }
        MatrixStream { chars: cvec, pos: (_pos, 0) }
    }

    fn age(&mut self) {
        self.chars.insert(0, thread_rng().gen_range(33..127) as u8 as char);
        self.chars.pop(); // Remove last character in trail
        
        self.pos.1 += 1; // Go down 1 y
    }

    fn getColor(&self, i: usize) -> Color {
        if i == 0 {
            Color::Rgb(244, 255, 244)
        }else{
            let step = 255/self.chars.len();
            Color::Rgb(0, 255 - (i*step) as u8, 0)
        }
    }
}

#[derive(Clone)]
pub struct MatrixWidget {
    area: Rect,
    matrix: Vec<MatrixStream>,
    /// The chance, between 0.0 and 1.0 that for each column on the widget, a stream will fall each
    /// draw loop.
    fall_rate: f64,
    max_dim: (u16, u16),
}

impl MatrixWidget {
    pub fn new(size: Rect) -> MatrixWidget {
        MatrixWidget { matrix: Vec::new(), fall_rate: 0.0125, area: size, max_dim: (size.width, size.height) }
    }

    pub fn populate(&mut self) {
        let mut rng = thread_rng();
        for i in 0..self.area.width {
            if rng.gen::<f64>() < self.fall_rate {
                self.matrix.push(MatrixStream::new(i as usize));
            }
        }
    }
    
    pub fn resize(&mut self, new_size: Rect) {
        if self.area.width < new_size.width {
            self.max_dim.0 = new_size.width;
        }
        if self.area.height < new_size.height {
            self.max_dim.1 = new_size.height;
        }

        self.area = new_size; 
    }

    pub fn age(&mut self) {
        for ms in self.matrix.iter_mut(){
            ms.age();
        }
        let (_, mh) = self.max_dim; 
        self.matrix.retain(|ms|{
            (ms.pos.1 as i32 - ms.chars.len() as i32) < mh as i32 // Keep all that return true
        });
    }
}

impl Widget for MatrixWidget {
    fn render(self, area: Rect, buf: &mut Buffer){
        for mstream in self.matrix.iter() {
            if mstream.pos.0 >= area.width as usize {
                continue;
            }
            

            for i in 0..mstream.chars.len() {
                let indx: i64 = mstream.pos.1 as i64 - i as i64;
                if indx >= area.height as i64 || indx < 0 {
                    continue;
                }

                let modif = if i == 0 { Modifier::UNDERLINED | Modifier::BOLD } else { Modifier::DIM };
                
                let color = mstream.getColor(i as usize);

                buf.get_mut(mstream.pos.0 as u16, indx as u16)
                    .set_style(Style::default()
                               .fg(color)
                               .add_modifier(modif)
                               )
                    .set_char(mstream.chars[i]);

            }
        }
    }
}
