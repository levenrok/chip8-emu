extern crate sdl2;

use std::env;
use std::fs;

use anyhow::{Result, bail};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

mod core;
mod rom;
use core::Core;
use rom::Rom;

macro_rules! init_or_bail {
    ($init: expr) => {
        match ($init) {
            Ok(v) => v,
            Err(e) => bail!(e),
        }
    };
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        bail!("specify a rom file to load!");
    }

    let filename = &args[1];

    let filedata: Vec<u8> = fs::read(filename)?;

    println!("loading ROM: \x1b[1m{}\x1b[0m", filename);

    let rom = {
        let name = filename.clone();
        let data = filedata;
        let size = data.len();

        Rom::new(name, data, size)
    };

    println!("name: {}, size: {}", rom.name(), rom.size);

    let mut emulator = Core::init(&rom);

    let sdl = init_or_bail!(sdl2::init());
    let video = init_or_bail!(sdl.video());

    let window = init_or_bail!(
        video
            .window("Chip-8 Emulator", 1024, 512)
            .position_centered()
            .build()
    );

    let mut canvas = init_or_bail!(window.into_canvas().build());
    let texture_creator = canvas.texture_creator();

    let texture =
        init_or_bail!(texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, 64, 32));

    let mut event_pump = init_or_bail!(sdl.event_pump());

    'keep_open: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => break 'keep_open,
                _ => {}
            }
        }

        emulator.cycle();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let dest = Rect::new(0, 0, 1024, 512);
        canvas.copy(&texture, None, Some(dest)).unwrap();

        canvas.present();

        std::thread::sleep(std::time::Duration::from_nanos(12 * 1_000_000));
    }

    Ok(())
}
