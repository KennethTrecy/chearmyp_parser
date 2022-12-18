use crate::abstracts::{
	AbstractBoundary,
	AbstractBoundaryCollection,
	AbstractNode,
	AbstractNodeQueue,
	AbstractAttacherCollection,
	AbstractAttacherNode
};
use super::ScopeStack;

impl<T, U, V, W, X, Y> ScopeStack<T, U, V, W, X, Y>
where
	T: AbstractBoundary<usize>,
	U: AbstractBoundaryCollection<usize, T>,
	V: AbstractAttacherNode + From<X>,
	W: AbstractAttacherCollection<V>,
	X: AbstractNode<usize, T, usize, T, U, V, W, X, Y>,
	Y: AbstractNodeQueue<X> {
	/// Returns the topmost scope by minimizing the collection of scopes to scope level 0.
	pub fn finalize(mut self) -> Y {
		self.minimize_scope_level_by(0);
		self.scopes.pop().unwrap()
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec, VecDeque};
	use crate::node::Node;
	use super::ScopeStack;

	#[test]
	fn can_finalize_empty_memory() {
		let scope_stack = ScopeStack::<
			Range<usize>,
			Vec<Range<usize>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>
		>::new();

		let nodes = scope_stack.finalize();

		assert_eq!(nodes, VecDeque::new());
	}

	use crate::scope_stack::Fragment;

	#[test]
	fn can_finalize_complex_fragment() {
		let concept = 0..13;
		let mut scope_stack = ScopeStack::<
			Range<usize>,
			Vec<Range<usize>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>
		>::new();
		let fragment = Fragment::new_complex(concept.clone(), VecDeque::new());
		scope_stack.fragments.push(fragment);
		scope_stack.scopes.push(VecDeque::new());

		let expected_nodes = {
			let mut nodes = VecDeque::with_capacity(1);
			let node = Node::Complex(concept.clone(), VecDeque::new(), VecDeque::new());
			nodes.push_back(node);
			nodes
		};

		let nodes = scope_stack.finalize();

		assert_eq!(nodes, expected_nodes);
	}

	#[test]
	fn can_finalize_with_simplex_fragment() {
		let concept = 13..18;
		let mut scope_stack = ScopeStack::<
			Range<usize>,
			Vec<Range<usize>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>
		>::new();
		let fragment = Fragment::new_simplex(concept.clone(), VecDeque::new());
		scope_stack.fragments.push(fragment);

		let expected_nodes = {
			let mut nodes = VecDeque::with_capacity(1);
			let node = Node::Simplex(concept.clone(), VecDeque::new());
			nodes.push_back(node);
			nodes
		};

		let nodes = scope_stack.finalize();

		assert_eq!(nodes, expected_nodes);
	}

	#[test]
	fn can_finalize_with_multilevel_tree() {
		let first_concept = 18..22;
		let second_concept = 22..26;
		let mut scope_stack = ScopeStack::<
			Range<usize>,
			Vec<Range<usize>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>
		>::new();
		let fragment = Fragment::new_complex(first_concept.clone(), VecDeque::new());
		scope_stack.fragments.push(fragment);
		scope_stack.scopes.push(VecDeque::new());
		let fragment = Fragment::new_simplex(second_concept.clone(), VecDeque::new());
		scope_stack.fragments.push(fragment);

		let expected_nodes = {
			let mut nodes = VecDeque::with_capacity(1);
			let first_node = Node::Complex(first_concept.clone(), VecDeque::new(), {
				let mut nodes = VecDeque::with_capacity(1);
				let second_node = Node::Simplex(second_concept.clone(), VecDeque::new());
				nodes.push_back(second_node);
				nodes
			});
			nodes.push_back(first_node);
			nodes
		};

		let nodes = scope_stack.finalize();

		assert_eq!(nodes, expected_nodes);
	}
}
