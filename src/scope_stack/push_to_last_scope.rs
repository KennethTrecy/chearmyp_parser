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
	V: AbstractAttacherNode,
	W: AbstractAttacherCollection<T, V>,
	X: AbstractNode<usize, T, usize, T, U, T, V, W, X, Y>,
	Y: AbstractNodeQueue<X> {
	pub fn push_to_last_scope(&mut self, node: X) {
		let last_scope = self.scopes.last_mut().unwrap();
		last_scope.push_node(node);
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec, VecDeque};
	use crate::node::Node;
	use crate::scope_stack::Relationship;
	use super::ScopeStack;

	#[test]
	fn can_push_to_last_scope() {
		let concept = 0..4;
		let node = Node::<Range<usize>, Vec<Range<usize>>>::Complex(
			concept.clone(),
			VecDeque::new(),
			VecDeque::new());

		let mut scope_stack = ScopeStack::new();

		let mut expected_scopes = Vec::new();
		let expected_last_scope = {
			let mut scope = VecDeque::new();
			let expected_complex = Node::Complex(concept.clone(), VecDeque::new(), VecDeque::new());
			scope.push_back(expected_complex);
			scope
		};
		expected_scopes.push(expected_last_scope);

		scope_stack.push_to_last_scope(node);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
