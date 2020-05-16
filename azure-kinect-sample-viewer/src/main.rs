fn main() {
    if let Err(e) = main2() {
        println!("{:?}", e);
    }
}

fn main2() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()?;

    Ok(())
}