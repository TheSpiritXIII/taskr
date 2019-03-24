use std::collections::{HashMap, HashSet};

use crate::task::Task;

struct TaskNode
{
	task: Task,
	depends_on: HashSet<&'static str>,
	depended_by: Vec<&'static str>,
}

impl TaskNode
{
	fn new(task: Task) -> Self
	{
		let depends_on = task.dependencies.clone();
		Self
		{
			task,
			depends_on,
			depended_by: Vec::new(),
		}
	}
}

pub struct Graph
{
	edges: Vec<&'static str>,
	node_map: HashMap<&'static str, TaskNode>,
}

impl Graph
{
	pub fn new(tasks: Vec<Task>) -> Result<Self, String>
	{
		let mut edges = Vec::new();

		let mut node_map: HashMap<&'static str, TaskNode> = HashMap::new();
		for task in tasks
		{
			let name = task.name;
			if task.dependencies.is_empty()
			{
				edges.push(name);
			}
			else
			{
				for dependency in &task.dependencies
				{
					if let Some(node) = node_map.get_mut(dependency)
					{
						node.depended_by.push(name);
					}
					else
					{
						return Err(format!("Could not resolve dependency `{}`. Please check order.", dependency));
					}
				}
			}

			let node = TaskNode::new(task);
			if let Some(_) = node_map.insert(name, node)
			{
				return Err(format!("Duplicate Task `{}`", name));
			}
		}

		Ok(Self
		{
			edges,
			node_map,
		})
	}

	pub fn resolve(&mut self, consumer: fn(Task))
	{
		loop
		{
			if let Some(edge) = self.edges.pop()
			{
				let node = self.node_map.remove(edge).unwrap();
				let name = node.task.name;
				consumer(node.task);
				for depended in node.depended_by
				{
					let depends_node = self.node_map.get_mut(depended).unwrap();
					depends_node.depends_on.remove(name);
					if depends_node.depends_on.is_empty()
					{
						self.edges.push(depends_node.task.name);
					}
				}
			}
			else
			{
				break;
			}
		}
	}
}

#[cfg(test)]
mod tests
{
	use crate::graph::Graph;
	use crate::task::Task;

	#[test]
	fn single_task()
	{
		let graph = Graph::new(vec!
		[
			Task::new("a", || {}),
		]);
		assert!(graph.is_ok());
	}

	#[test]
	fn multi_dependency()
	{
		let graph = Graph::new(vec!
		[
			Task::new("a", || {}),
			Task::with_dependencies("b", &["a"], || {}),
			Task::with_dependencies("c", &["b"], || {}),
		]);
		assert!(graph.is_ok());
	}

	#[test]
	fn missing_dependency()
	{
		let graph = Graph::new(vec!
		[
			Task::with_dependencies("a", &["b"], || {})
		]);
		assert!(graph.is_err());
	}

	#[test]
	fn duplicate_task()
	{
		let graph = Graph::new(vec!
		[
			Task::new("a", || {}),
			Task::with_dependencies("a", &["b"], || {}),
		]);
		assert!(graph.is_err());
	}

	#[test]
	fn cyclic_dependency()
	{
		let graph = Graph::new(vec!
		[
			Task::with_dependencies("a", &["b"], || {}),
			Task::with_dependencies("b", &["a"], || {}),
		]);
		assert!(graph.is_err());
	}
}
