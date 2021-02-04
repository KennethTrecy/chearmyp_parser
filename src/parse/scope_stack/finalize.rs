use alloc::vec::Vec;
use crate::Node;
use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	/// Returns the topmost scope by minimizing the collection of scopes to scope level 0.
	pub fn finalize(mut self) -> Vec<Node<'a>> {
		self.minimize_scope_level_by(0);
		self.scopes.pop().unwrap()
	}
}

#[cfg(test)]
mod t {
	use crate::parse::scope_stack::Fragment;
	use super::{Node, ScopeStack, Vec};

	#[test]
	fn can_finalize_empty_memory() {
		let scope_stack = ScopeStack::new();

		let nodes = scope_stack.finalize();

		assert_eq!(nodes, Vec::new());
	}

	#[test]
	fn can_finalize_complex_fragment() {
		let concept = b"abcdefghijklm";
		let mut scope_stack = ScopeStack::new();
		let fragment = Fragment::Complex(&concept[..], Vec::new());
		scope_stack.fragments.push(fragment);
		scope_stack.scopes.push(Vec::new());

		let expected_nodes = {
			let mut nodes = Vec::with_capacity(1);
			let node = Node::Complex(&concept[..], Vec::new(), Vec::new());
			nodes.push(node);
			nodes
		};

		let nodes = scope_stack.finalize();

		assert_eq!(nodes, expected_nodes);
	}

	#[test]
	fn can_finalize_with_simplex_fragment() {
		let concept = b"nopqr";
		let mut scope_stack = ScopeStack::new();
		let fragment = Fragment::Simplex(&concept[..], Vec::new());
		scope_stack.fragments.push(fragment);

		let expected_nodes = {
			let mut nodes = Vec::with_capacity(1);
			let node = Node::Simplex(&concept[..], Vec::new());
			nodes.push(node);
			nodes
		};

		let nodes = scope_stack.finalize();

		assert_eq!(nodes, expected_nodes);
	}

	#[test]
	fn can_finalize_with_multilevel_tree() {
		let first_concept = b"stuv";
		let second_concept = b"wxyz";
		let mut scope_stack = ScopeStack::new();
		let fragment = Fragment::Complex(&first_concept[..], Vec::new());
		scope_stack.fragments.push(fragment);
		scope_stack.scopes.push(Vec::new());
		let fragment = Fragment::Simplex(&second_concept[..], Vec::new());
		scope_stack.fragments.push(fragment);

		let expected_nodes = {
			let mut nodes = Vec::with_capacity(1);
			let first_node = Node::Complex(&first_concept[..], Vec::new(), {
				let mut nodes = Vec::with_capacity(1);
				let second_node = Node::Simplex(&second_concept[..], Vec::new());
				nodes.push(second_node);
				nodes
			});
			nodes.push(first_node);
			nodes
		};

		let nodes = scope_stack.finalize();

		assert_eq!(nodes, expected_nodes);
	}
}
