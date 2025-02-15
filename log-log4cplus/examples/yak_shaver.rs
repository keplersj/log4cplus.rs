use std::fmt::Display;

use log::{debug, error, info, trace, warn};
use log4cplus::{BasicConfigurator, Initializer, Logger};
use rand::Rng;

struct Razor;

impl Razor {
    fn find() -> Option<Self> {
        trace!("Locating a razor");
        if rand::thread_rng().gen_ratio(1, 20) {
            debug!("You got a Nat 20!");
            Some(Razor)
        } else {
            error!("Reroll Required");
            None
        }
    }
}

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

pub fn shave_the_yak(yak: &mut Yak) {
    trace!("Commencing yak shaving");

    loop {
        match Razor::find() {
            Some(razor) => {
                info!("Razor located: {razor}");
                yak.shave(razor);
                break;
            }
            None => {
                warn!("Unable to locate a razor, retrying");
            }
        }
    }
}

fn main() -> Result<(), log::SetLoggerError> {
    let _initializer = Initializer::new();

    let mut configurator = BasicConfigurator::new();
    configurator.configure();

    let logger = Logger::get_instance("yak_shaver");
    log_log4cplus::Log4CPlusLogger::from(logger).try_init()?;

    let mut yak = Yak;
    shave_the_yak(&mut yak);

    Ok(())
}
