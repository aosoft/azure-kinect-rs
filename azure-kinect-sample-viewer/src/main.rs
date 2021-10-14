use azure_kinect::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

fn main() {
    if let Err(e) = main2() {
        println!("{:?}", e);
    }
}

fn main2() -> Result<(), Box<dyn std::error::Error>> {
    let factory = Factory::new()?;
    let device = factory.device_open(0)?;
    let camera_config = DeviceConfiguration::builder()
        .depth_mode(DepthMode::NFov2x2Binned)
        .camera_fps(Fps::_30fps)
        .build();
    let camera = device.start_cameras(&camera_config)?;

    #[cfg(feature = "depth-view")]
    let image_dimension = camera_config.depth_mode().get_dimension();
    #[cfg(not(feature = "depth-view"))]
    let image_dimension = camera_config.color_resolution.get_dimension();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "azure-kinect-sample-viewer",
            image_dimension.width as u32,
            image_dimension.height as u32,
        )
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::ARGB8888,
            image_dimension.width as u32,
            image_dimension.height as u32,
        )
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if let Ok(capture) = camera.get_capture(1) {
            texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                #[cfg(feature = "depth-view")]
                {
                    let depth_image = capture.get_depth_image();
                    let width = depth_image.get_width_pixels();
                    unsafe {
                        for y in 0..depth_image.get_height_pixels() as usize {
                            let p = depth_image
                                .get_buffer()
                                .add(y * depth_image.get_stride_bytes() as usize)
                                as *const u16;
                            let p2 = buffer.as_mut_ptr().add(y * pitch) as *mut u32;
                            for x in 0..width as isize {
                                let value = *p.offset(x);
                                *p2.offset(x) =
                                    get_depth_color(value, camera_config.depth_mode().get_range())
                            }
                        }
                    }
                }

                #[cfg(not(feature = "depth-view"))]
                {
                    let image = capture.get_color_image();
                    let width = image.get_width_pixels();
                    for y in 0..image.get_height_pixels() as usize {
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                image
                                    .get_buffer()
                                    .add(y * image.get_stride_bytes() as usize),
                                buffer.as_mut_ptr().add(y * pitch),
                                (width * 4) as usize,
                            );
                        }
                    }
                }
            })?;
            canvas.clear();
            canvas.copy(&texture, None, None)?;
            canvas.present();
        }
    }

    Ok(())
}

fn get_depth_color(depth: u16, minmax: Range<u16>) -> u32 {
    if depth == 0 {
        return 0xff000000;
    }

    let clamped_value = std::cmp::min(minmax.max, std::cmp::max(depth, minmax.min));

    const RANGE: f32 = 2.0 / 3.0;
    let hue =
        RANGE - (clamped_value - minmax.min) as f32 / (minmax.max - minmax.min) as f32 * RANGE;

    let i = (hue * 6.0) as i32;
    let f = hue * 6.0 - i as f32;

    let rgb = match i {
        0 => (1.0f32, f, 0.0f32),
        1 => (1.0 - f, 1.0f32, 0.0f32),
        2 => (0.0f32, 1.0f32, f),
        3 => (0.0f32, 1.0 - f, 1.0f32),
        4 => (f, 0.0f32, 1.0f32),
        _ => (1.0f32, 0.0f32, 1.0 - f),
    };

    0xff000000
        | (((255.0 * rgb.0) as u32) << 16)
        | (((255.0 * rgb.1) as u32) << 8)
        | ((255.0 * rgb.2) as u32)
}
