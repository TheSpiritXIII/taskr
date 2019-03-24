use std::collections::HashSet;
use std::iter::FromIterator;

enum Task

pub struct Task
{
	pub name: &'static str,
	pub dependencies: HashSet<&'static str>,
	pub run: fn() -> (),
}

impl Task
{
	pub fn new(name: &'static str, run: fn() -> ()) -> Self
	{
		Self
		{
			name,
			dependencies: HashSet::new(),
			run,
		}
	}
	pub fn with_dependencies(name: &'static str, dependencies: &[&'static str], run: fn() -> ()) -> Self
	{
		Self
		{
			name,
			dependencies: HashSet::from_iter(dependencies.iter().cloned()),
			run,
		}
	}
}
