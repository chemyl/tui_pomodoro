// managing app theme
use cursive::theme::{BaseColor, Color, PaletteColor, Theme};
//import interface struct elements
use cursive::views::{Dialog, TextView};
//import main app object & additional extension for managing flow
use cursive::{Cursive, CursiveExt};

use cursive::view::Nameable;
// add sync smart-pointers
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Cursive Object
    let mut siv = Cursive::default();
    siv.set_theme(custom_theme());

    // App Variables
    let pomodoro_duration = Arc::new(Mutex::new(25 * 60));
    let break_duration = Arc::new(Mutex::new(5 * 60));
    let time_countdown = Arc::new(Mutex::new(*pomodoro_duration.lock().unwrap()));

    let is_break_time = Arc::new(Mutex::new(false));
    let is_running = Arc::new(Mutex::new(false));

    //cloning the variables to pass into threads where they will be accessed safely and modified
    let time_countdown_clone = Arc::clone(&time_countdown);

    // create textview to display time left on the timer
    let text_view = TextView::new
        (format!("Time: {}", time_formatter(*time_countdown.lock().unwrap()))).with_name("Timer");

    // TUI
    siv.add_layer
    (Dialog::around(text_view)
         .button("+ 1 min", {
             // create a clone of the variables
             let pomodoro_duration = Arc::clone(&pomodoro_duration);
             let time_countdown = Arc::clone(&time_countdown);
             // closures with data transfer by move
             move |cursive| {
                 let mut duration = pomodoro_duration.lock().unwrap();
                 *duration += 60;
                 let mut timer_duration = time_countdown.lock().unwrap();
                 *timer_duration = *duration;
                 // textView updating
                 cursive.call_on_name("Timer", |view: &mut TextView| {
                     view.set_content(format!("Time: {}", time_formatter(*duration)));
                 });
             }
         })
         .button("- 1 min", {
             let pomodoro_duration = Arc::clone(&pomodoro_duration);
             let time_countdown = Arc::clone(&time_countdown);
             move |cursive| {
                 let mut duration = pomodoro_duration.lock().unwrap();
                 if *duration > 60 {
                     *duration -= 60;
                 }
                 let mut timer_duration = time_countdown.lock().unwrap();
                 *timer_duration = *duration;
                 cursive.call_on_name("Timer", |view: &mut TextView| {
                     view.set_content(format!("Time: {}", time_formatter(*duration)));
                 });
             }
         })
         .button("Start|Stop", {
             let is_running_clone = Arc::clone(&is_running);
             move |cursive| {
                 let mut running = is_running_clone.lock().unwrap();
                 *running = !*running;
                 if *running {
                     cursive.call_on_name("Timer", move |view: &mut TextView| {
                         view.set_content("Timer is running...");
                     });
                 } else {
                     cursive.call_on_name("Timer", move |view: &mut TextView| {
                         view.set_content("Timer is paused...");
                     });
                 }
             }
         })
         .button("Exit", |cursive| cursive.quit()).title("Pomodoro Timer"), );

    // Threads
    // First thread handles countdown of the timer (background thread)
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            let mut time_left = time_countdown_clone.lock().unwrap();
            let mut break_time = is_break_time.lock().unwrap();
            let running = is_running.lock().unwrap();
            if *running {
                if *time_left > 0 {
                    *time_left -= 1;
                } else {
                    if *break_time {
                        *time_left = *pomodoro_duration.lock().unwrap();
                        *break_time = false;
                        println!("Break is finished!");
                    } else {
                        *time_left = *break_duration.lock().unwrap();
                        *break_time = true;
                        println!("  Its break time!");
                    }
                }
            }
        }
    });

    // Second thread handles refreshing of the UI every seconds
    let cb_sink = siv.cb_sink().clone();
    let timer_counter_for_refresh = Arc::clone(&time_countdown);

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));

        let timer_countdown_refresh = Arc::clone(&timer_counter_for_refresh);
        cb_sink.send(Box::new(move |cursive: &mut Cursive| {
            let time_left = *timer_countdown_refresh.lock().unwrap();
            cursive.call_on_name("Timer", move |view: &mut TextView| {
                view.set_content(format!("Time: {}", time_formatter(time_left)));
            });
        })).unwrap();
    });
    siv.run();
}

fn time_formatter(time: usize) -> String {
    let minutes = time / 60;
    let seconds = time % 60;
    format!("{}:{:02}", minutes, seconds)
}

fn custom_theme() -> Theme {
    let mut theme = Theme::retro();
    theme.palette[PaletteColor::Background] = Color::Light(BaseColor::Cyan);
    theme.palette[PaletteColor::View] = Color::Light(BaseColor::White);
    theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Secondary] = Color::Dark(BaseColor::Red);
    theme.palette[PaletteColor::Highlight] = Color::Light(BaseColor::Green);
    theme
}