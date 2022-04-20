#[path = "learn/learn.rs"]
mod learn;
use learn::*;

#[path = "run/run.rs"]
mod run;
use run::run;

pub fn run() {
	// Run
	run::run();
}