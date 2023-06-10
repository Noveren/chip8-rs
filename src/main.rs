#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod vm;

use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
    rect::Rect,
};
use std::time::Duration;

const SQUARE_SIZE:       u32 = 8;
const PLAYGROUND_HEIGHT: u32 = 32;
const PLAYGROUND_WIDTH:  u32 = 64;

const FONT_SET: [u8; 16 * 5] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];



fn main() -> Result<(), String> {
    let mut vm: vm::VM = Default::default();
    vm.load_fontset(&FONT_SET).map_err(|e| e.to_string())?;
    vm.load_rom(&[
        0x60, 0x01,
        0x61, 0x01,
        0xd0, 0x15,

        0xa0, 0x05,

        0x60, 0x11,
        0x61, 0x11,
        0xd0, 0x15,
    ]).map_err(|e| e.to_string())?;
    vm.step().map_err(|e| e.to_string())?;
    vm.step().map_err(|e| e.to_string())?;
    vm.step().map_err(|e| e.to_string())?;
    vm.step().map_err(|e| e.to_string())?;
    vm.step().map_err(|e| e.to_string())?;
    vm.step().map_err(|e| e.to_string())?;
    vm.step().map_err(|e| e.to_string())?;
    // println!("{}", vm);

    let sdl_context = sdl2::init()?;
    let vidieo_subsystem = sdl_context.video()?;

    let window = vidieo_subsystem
        .window("Chip8", SQUARE_SIZE * PLAYGROUND_WIDTH, SQUARE_SIZE * PLAYGROUND_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut renderer = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // println!("Using SDL_Renderer \"{}\"", renderer.info().name);

    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    // 定义并初始化一种 Texture
    let texture_creator = renderer.texture_creator();
    let mut texture = texture_creator.create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE).map_err(|e| e.to_string())?;
    renderer.with_texture_canvas(&mut texture, |texture_canvas| {
        texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
        texture_canvas.clear();
        texture_canvas.draw_rect(Rect::new(0, 0, SQUARE_SIZE, SQUARE_SIZE))
            .expect("Could not draw rect");
    }).map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // 事件处理
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // 逻辑执行

        // 渲染执行
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        for (i, v) in vm.get_gfx().iter().enumerate() {
            if *v != 0x00 {
                let x: i32 = (i % (PLAYGROUND_WIDTH as usize) * (SQUARE_SIZE as usize)).try_into().unwrap();
                let y: i32 = (i / (PLAYGROUND_WIDTH as usize) * (SQUARE_SIZE as usize)).try_into().unwrap();
                renderer.copy(&texture, None, Rect::new(
                    x, y,
                    SQUARE_SIZE, SQUARE_SIZE
                ))?;
            }
        }

        renderer.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}