mod store;
mod gen;
mod data;
mod systems;

use std::io::Write;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use store::EcsStore;

fn main() {
    // get keyboard input in a thready manner
    let (ch_s, ch_r) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        for k in stdin.keys() { //keys depend on TermRead trait
            ch_s.send(k).ok();
        }
    });
    
    let (w, h) = termion::terminal_size().unwrap();
    let (w, h) = (w as i32, h as i32);
    let mut screen = std::io::stdout().into_raw_mode().unwrap();
    let mut gen = gen::GenManager::new();
    let mut strengths = store::VecStore::new();
    let mut dirs = store::VecStore::new();
    let mut poss = store::VecStore::new();
    let mut pass = 0;
    
    loop {
        // create one element per loop 
        let g = gen.next();
        strengths.add(g, data::Strength{s:1, h: 5});
        dirs.add(g, data::Dir{vx: 0, vy: 0});
        poss.add(g, data::Pos{x: (rand::random::<i32>() % w), y: (rand::random::<i32>() % h)});

        systems::dir_sys(&mut dirs, &poss);
        systems::move_sys(&dirs, &mut poss);
        systems::collision_sys(&poss, &mut strengths);
        systems::death_sys(&mut gen, &mut strengths, &mut poss, &mut dirs);
        systems::render_sys(&mut screen, &poss, &strengths);

        write!(&mut screen, "{}Pass={}", termion::cursor::Goto(1,1), pass).ok();
        pass += 1;
        screen.flush().ok();

        while let Ok(Ok(k)) = ch_r.try_recv() {
            match k {
                Key::Char('q') => return,
                // here handle any key presses to make the game do stuff
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(300));
    }

}
