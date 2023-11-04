// use crate::frame::Frame;
use crossterm::event;
use crossterm::event::Event;
use crossterm::{
    cursor::{Hide, Show},
    event::KeyCode,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::frame::{new_frame, Drawable};
use invaders::{frame, render};
use rusty_audio::Audio;
use std::sync::mpsc;
use std::thread;
use std::{error::Error, io, time::Duration};
mod player; // Import the player module

use player::Player; // Import the Player struct
                    // Create a new audio context.

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    // Add audio files to the context.
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");

    // Play the "startup" sound.
    audio.play("startup");

    //terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();

    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &last_frame, false);
            last_frame = curr_frame;
        }
    });

    //Game Loop
    let mut player = Player::new();
    'gameloop: loop {
        //Per-frame init
        let mut curr_frame = new_frame();

        //input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        //Draw & render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    //cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    audio.wait();
    Ok(())
}
