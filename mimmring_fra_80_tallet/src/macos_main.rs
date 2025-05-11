use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use sdl2::{mixer, Sdl};
use rand::Rng;
use std::time::Instant;

fn init_sdl2() -> (Sdl, sdl2::ttf::Sdl2TtfContext) {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    (sdl_context, ttf_context)
}

fn create_canvas(sdl_context: &Sdl) -> Canvas<Window> {
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Mimmring fra 80 tallet", 640, 520).position_centered().build().unwrap();
    window.into_canvas().present_vsync().build().unwrap()
}

fn gradient(b: Color) -> [Color; 13] {
    let mut g = [Color::RGB(0, 0, 0); 13];
    for i in 0..6 {
        let f = (i * 40) as u8;
        g[i] = Color::RGB(b.r.saturating_add(f), b.g.saturating_add(f), b.b.saturating_add(f));
        g[12 - i] = g[i];
    }
    g[6] = Color::RGB(255, 255, 255);
    g
}

fn bars(c: &mut Canvas<Window>, w: u32, h: u32, t: f32, ty: i32, by: i32) {
    let col = [Color::RGB(255, 255, 85), Color::RGB(255, 64, 64), Color::RGB(85, 85, 255), Color::RGB(139, 69, 19)];
    let bh = h / 4;
    for (i, b) in col.iter().enumerate() {
        let g = gradient(*b);
        let off = (t * 2.0 + i as f32 * 0.5).sin() * 40.0;
        let boff = -off;
        for k in 0..13 {
            let y = k as f32 + off;
            if y >= 0.0 && y < bh as f32 {
                c.set_draw_color(g[k]);
                let _ = c.fill_rect(Rect::new(0, y as i32 + ty, w, 1));
            }
            let yb = k as f32 + boff + by as f32;
            if yb >= by as f32 && yb < by as f32 + bh as f32 {
                c.set_draw_color(g[k]);
                let _ = c.fill_rect(Rect::new(0, yb as i32, w, 1));
            }
        }
    }
}

fn scroller<'a>(c: &mut Canvas<Window>, tc: &'a TextureCreator<WindowContext>, f: &Font, txt: &str, off: f32, y: i32) {
    let surf = f.render(txt).blended(Color::RGB(255, 255, 255)).unwrap();
    let tex = tc.create_texture_from_surface(&surf).unwrap();
    let TextureQuery { width, height, .. } = tex.query();
    let sx = (off as i32 % width as i32) - width as i32;
    let _ = c.copy(&tex, None, Some(Rect::new(sx, y, width, height)));
    let _ = c.copy(&tex, None, Some(Rect::new(sx + width as i32, y, width, height)));
}

fn stars(c: &mut Canvas<Window>, v: &[(i32, i32, Color)]) {
    for &(x, y, col) in v {
        c.set_draw_color(col);
        let _ = c.fill_rect(Rect::new(x, y, 2, 2));
    }
}

fn sprite(c: &mut Canvas<Window>, tc: &TextureCreator<WindowContext>, x: f32, y: f32) {
    let surf = sdl2::surface::Surface::load_bmp("assets/sprite.bmp").unwrap();
    let tex = tc.create_texture_from_surface(&surf).unwrap();
    let TextureQuery { width, height, .. } = tex.query();
    let _ = c.copy(&tex, None, Some(Rect::new(x as i32, y as i32, width, height)));
}

fn main() {
    if mixer::init(mixer::InitFlag::MP3 | mixer::InitFlag::OGG).is_err() { return; }
    if mixer::open_audio(44_100, mixer::AUDIO_S16LSB, 2, 1024).is_err() { return; }
    mixer::allocate_channels(4);
    let mus = mixer::Music::from_file("assets/music.wav").unwrap();
    mus.play(-1).unwrap();
    let (sdl, ttf) = init_sdl2();
    let mut cv = create_canvas(&sdl);
    let tc = cv.texture_creator();
    let f = ttf.load_font(r"/System/Library/Fonts/Supplemental/Arial.ttf", 24).unwrap();
    let mut ev = sdl.event_pump().unwrap();
    let st = Instant::now();
    let top = "        Dette er en liten test laget med SDL i Rust – et lite tilbakeblikk på ungdomstiden og gamle spillminner. Tenk Tiki-100 og Commodore 64! En hyllest til den tida da alt handlet om kreativitet, nysgjerrighet og pixler i bevegelse.        ";
    let bot = "          Jeg husker NinjaWriter som en avansert teksteditor på Commodore 64 – perfekt for å lage imponerende tekst. Vi brukte den til å skrive meldinger til hverandre i hverdagen, og diskettene ble nesten glødende i posten. Man fant på mye rart den gangen! Det var spennende å eksperimentere med sprites, inspirert av Commodore 64 Programmer's Reference Guide, den gamle programmeringsboka vi alltid hadde for hånden. Enten det var i BASIC eller 6502 assembler, handlet det om å prøve, feile – og lære.        ";
    let mut rng = rand::thread_rng();
    let stars_v: Vec<(i32, i32, Color)> = (0..100).map(|_| (rng.gen_range(0..640), rng.gen_range(0..520), Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255)))).collect();
    let r = 100.0;
    let cx = 320.0;
    let cy = 240.0;
    let mut ang: f32 = 0.0;
    loop {
        for e in ev.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = e { return; }
        }
        let t = st.elapsed().as_secs_f32();
        cv.set_draw_color(Color::RGB(0, 0, 0));
        cv.clear();
        bars(&mut cv, 640, 480, t, 0, 460);
        scroller(&mut cv, &tc, &f, top, -t * 100.0, 10);
        scroller(&mut cv, &tc, &f, bot, t * 100.0, 460);
        stars(&mut cv, &stars_v);
        let sx = cx + r * ang.cos();
        let sy = cy + r * ang.sin();
        sprite(&mut cv, &tc, sx, sy);
        cv.present();
        ang += 0.02;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
