// managing app theme
use cursive::theme::{BaseColor, Color, PaletteColor, Theme};
//import interface struct elements
use cursive::views::{Dialog, TextView};
//import main app object & additional extension for managing flow
use cursive::{Cursive, CursiveExt};

// add sync smart-pointers
use std::sync::{Arc, Mutex};

use cursive::view::Nameable;

fn main() {
    // create cursive main object
    let mut siv = Cursive::default();
    // set theme
    siv.set_theme(custom_theme());

    // create base variables
    let pomodoro_duration = Arc::new(Mutex::new(25 * 60));
    let break_duration = Arc::new(Mutex::new(5 * 60));
    // показать обновленную продолжительность
    // *pomodoro_duration.lock()
    // * - чтобы обновить продолжительность продолджительности в текущем таймере
    // lock() - чтобы заблокировать на этот момент
    let time_countdown = Arc::new(Mutex::new(*pomodoro_duration.lock().unwrap()));

    // two flags
    let is_break_time = Arc::new(Mutex::new(false));
    let is_running = Arc::new(Mutex::new(false));

    //cloning the variables to pass into threads where they will be accessed safely and modified
    let time_countdown_clone = Arc::clone(&time_countdown);
    let pomodoro_duration_clone = Arc::clone(&pomodoro_duration);
    let break_duration_clone = Arc::clone(&break_duration);

    // create textview to displat timeleft on the timer
    // MM:SS
    let text_view = TextView::new
        (format!(
            "Time: {}", time_formatter(*time_countdown.lock().unwrap())
        )).with_name("Timer");


    // create TUI
    siv.add_layer(Dialog::around(text_view).button("add 1 minute", {
        let pomodoro_duration = Arc::clone(&pomodoro_duration);
        let time_countdown = Arc::clone(&time_countdown);
        move |x| {
            let mut duration = pomodoro_duration.lock().unwrap();
            *duration += 60;
            let mut timer_duration = time_countdown.lock().unwrap();
            *timer_duration = *duration;
            x.call_on_name("Timer", |view: &mut TextView| {
                view.set_content(format!("Time: {}", time_formatter(*duration)));
            });
        }
    }));


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
