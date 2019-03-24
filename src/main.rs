#![feature(nll)]
#![allow(unused_variables, dead_code)]

mod graph;
mod runner;
mod task;

use runner::Runner;
use task::Task;

fn main()
{
	Runner::run(vec![
		Task::new("java", || {
			// TODO.
		}),
		Task::with_dependencies("jar", &["java"], || {
			// TODO
		}),
	]);
}
