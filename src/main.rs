extern crate sdl2;

mod events;

use events::Events;
use sdl2::pixels::Color;

fn main() {
    //init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    //window
    let window = video.window("ArcadeRS Shooter", 800, 600).position_centered().opengl().build().unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();

    //events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        //render
        renderer.set_draw_color(Color::RGB(0,0,0));
        renderer.clear();
        renderer.present();
    }
}
