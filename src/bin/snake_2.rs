use rand::Rng;
use rust_nes::{cpu::Cpu, rom::Rom};
use sdl2::{pixels::{PixelFormatEnum, Color}, EventPump, event::Event, keyboard::Keycode};

const SCALE: u32 = 10;

fn handle_user_input(cpu: &mut Cpu, event_pump: &mut EventPump)
{
    for event in event_pump.poll_iter()
    {
        match event
        {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0);
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => cpu.memory.write(0xFF, 0x77),
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => cpu.memory.write(0xFF, 0x73),
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => cpu.memory.write(0xFF, 0x61),
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => cpu.memory.write(0xFF, 0x64),
            _ => {}
        }
    }
}

fn sdl2_color(byte: u8) -> Color {
    match byte {
        0 => sdl2::pixels::Color::BLACK,
        1 => sdl2::pixels::Color::WHITE,
        2 | 9 => sdl2::pixels::Color::GREY,
        3 | 10 => sdl2::pixels::Color::RED,
        4 | 11 => sdl2::pixels::Color::GREEN,
        5 | 12 => sdl2::pixels::Color::BLUE,
        6 | 13 => sdl2::pixels::Color::MAGENTA,
        7 | 14 => sdl2::pixels::Color::YELLOW,
        _ => sdl2::pixels::Color::CYAN,
    }
 }

fn screen_changed(cpu: &Cpu, state: &mut [u8; 32 * 3 * 32]) -> bool
{
    let mut update = false;

    for pixel_addr in 0x0200..0x0600
    {
        let state_idx = ((pixel_addr - 0x0200) * 3) as usize;
        let color = cpu.memory.read(pixel_addr);
        let color_rgb = sdl2_color(color).rgb();

        if state[state_idx] != color_rgb.0 ||
           state[state_idx + 1] != color_rgb.1 ||
           state[state_idx + 2] != color_rgb.2
        {
            update = true;

            state[state_idx] = color_rgb.0;
            state[state_idx + 1] = color_rgb.1;
            state[state_idx + 2] = color_rgb.2;
        }
    }

    update
}

fn main()
{
    // Init SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Snake", 32 as u32 * SCALE, 32 as u32 * SCALE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture_target(PixelFormatEnum::RGB24, 32, 32).unwrap();

    let mut cpu = Cpu::new();
    let mut rng = rand::thread_rng();

    let mut screen_state = [0 as u8; 32 * 3 * 32];

    canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();

    let rom = Rom::from_file("./resources/snake.nes").unwrap();

    cpu.load_rom(rom);
    cpu.reset();

    cpu.run(|cpu: &mut Cpu|{
        handle_user_input(cpu, &mut event_pump);

        cpu.memory.write(0xFE, rng.gen_range(1..16));

        if screen_changed(&cpu, &mut screen_state)
        {
            texture.update(None, &screen_state, 32 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        ::std::thread::sleep(std::time::Duration::new(0, 70_000));
    });
}