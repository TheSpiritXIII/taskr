use ansi_term::Colour::{Red, Green};
use ansi_term::Style;

use crate::task::Task;
use crate::graph::Graph;

pub struct Runner;

impl Runner
{
	fn print(text: &str, style: Style)
	{
		println!("{}", format!("{}", style.paint(format!("{:*<80}", format!("{} ", text)))));
	}
	fn print_ok()
	{
		println!("{}\n", Style::new().fg(Green).paint("[ok]"));
	}
	pub fn run(tasks: Vec<Task>)
	{
		if let Err(_) = ansi_term::enable_ansi_support()
		{
			println!("Unable to enable Windows ASSI code support. Colors will be disabled.\n");
		}

		Runner::print("BUILDING GRAPH", Style::new().bold());
		match Graph::new(tasks)
		{
			Ok(mut graph) =>
			{
				Runner::print_ok();
				graph.resolve(|task|
				{
					Runner::print(&format!("TASK: {}", task.name), Style::new().bold());
					(task.run)();
					Runner::print_ok();
				});
			}
			Err(e) =>
			{
				println!("This is in red: {}", Red.paint("a red string"));
			}
		}
	}
}
