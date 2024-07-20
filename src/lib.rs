#![no_std]

mod settings;
mod state;
mod trigger;

use settings::Settings;
use state::State;
use trigger::Triggers;

use asr::settings::gui::Gui;
use asr::timer::TimerState;
use asr::{future::next_tick, Process};

asr::async_main!(stable);
asr::panic_handler!();

trait Square {
    fn square(&self) -> f64;
}

impl Square for f64 {
    fn square(&self) -> f64 {
        self * self
    }
}

async fn main() {
    let mut settings = Settings::register();

    loop {
        settings.update();
        //Attempt to attach to process with the following name
        let process = Process::wait_attach("ChainedTogether-Win64-Shipping").await;
        process
            .until_closes(async {
                asr::print_message("Creating Initial State");

                let mut current_state = State::default();
                let mut current_trigger = Triggers::first_trigger(&settings);

                loop {
                    //Load each of the relevant values from memory in to variables
                    let timer_state = &asr::timer::state();
                    let old_state = current_state.clone();
                    current_state = match State::generate(&process) {
                        Ok(Some(state)) => state,
                        Ok(None) => {
                            continue;
                        }
                        Err(_) => {
                            continue;
                        }
                    };

                    asr::timer::set_game_time(asr::time::Duration::new(
                        current_state.timer as i64,
                        0,
                    ));

                    #[cfg(debug_assertions)]
                    current_state.log();

                    let unstarted_states = [
                        TimerState::NotRunning,
                        TimerState::Ended,
                        TimerState::Unknown,
                    ];

                    //Logic to trigger timer start if necessary
                    if unstarted_states.contains(timer_state)
                        && current_state.should_start(&old_state)
                    {
                        #[cfg(debug_assertions)]
                        asr::print_message("Starting run!");
                        asr::timer::start();
                    }

                    //Logic to trigger splits
                    if *timer_state == TimerState::Running
                        && current_state.should_split(&current_trigger)
                    {
                        #[cfg(debug_assertions)]
                        asr::print_limited::<1024>(&format_args!("Splitting!",));

                        asr::timer::split();
                        current_trigger = current_trigger.next_trigger(&settings);
                    }

                    //Logic to trigger Resets
                    if *timer_state != TimerState::NotRunning && current_state.should_reset() {
                        #[cfg(debug_assertions)]
                        asr::print_message("Reseting Run");

                        asr::timer::reset();
                        current_trigger = Triggers::first_trigger(&settings);
                    }

                    //Logic to reset current trigger to start so fresh splits can work
                    if *timer_state == TimerState::NotRunning {
                        current_trigger = Triggers::first_trigger(&settings);
                    }

                    next_tick().await;
                }
            })
            .await;
    }
}
