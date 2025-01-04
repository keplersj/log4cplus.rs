use std::fmt::Display;

use log::{debug, error, info, trace, warn};
use rand::Rng;

struct Razor;

impl Display for Razor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Razor")
    }
}

pub struct Yak;

impl Yak {
    fn shave(&mut self, _razor: Razor) {
        info!("Yak shaved");
    }
}

fn find_a_razor() -> Result<Razor, String> {
    trace!("Locating a razor");
    if rand::thread_rng().gen_ratio(1, 20) {
        debug!("You got a Nat 20!");
        Ok(Razor)
    } else {
        error!("Reroll Required");
        Err("Razor not found".to_string())
    }
}

pub fn shave_the_yak(yak: &mut Yak) {
    trace!("Commencing yak shaving");

    loop {
        match find_a_razor() {
            Ok(razor) => {
                info!("Razor located: {razor}");
                yak.shave(razor);
                break;
            }
            Err(err) => {
                warn!("Unable to locate a razor: {err}, retrying");
            }
        }
    }
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    let mut yak = Yak;
    shave_the_yak(&mut yak);
}
