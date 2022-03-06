use crate::abstracts::{
	AbstractBoundary,
	AbstractBoundaryCollection,
	AbstractNode,
	AbstractNodeQueue,
	AbstractAttacherCollection,
	AbstractAttacherNode
};
use crate::scope_stack::Fragment;
use super::ScopeStack;

impl<T, U, V, W, X, Y> ScopeStack<T, U, V, W, X, Y>
where
	T: AbstractBoundary<usize>,
	U: AbstractBoundaryCollection<usize, T>,
	V: AbstractAttacherNode + From<X>,
	W: AbstractAttacherCollection<T, V>,
	X: AbstractNode<usize, T, usize, T, U, T, V, W, X, Y>,
	Y: AbstractNodeQueue<X> {
	/// Promotes the last fragment into a node in the last scope.
	pub fn promote_last_fragment(&mut self) {
		let last_fragment = self.fragments.pop().unwrap();
		let node;

		match last_fragment {
			Fragment::Simplex(simplex_boundary, attachers, _) => {
				node = X::new_simplex(simplex_boundary, attachers);
			},
			Fragment::Complex(complex_boundary, attachers, _) => {
				let last_scope = self.scopes.pop().unwrap();
				node = X::new_complex(complex_boundary, attachers, last_scope);
			}
		}

		self.push_to_preferred_relationship(node);
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec, VecDeque};
	use crate::node::Node;
	use super::{Fragment, ScopeStack};

	#[test]
	#[should_panic]
	fn cannot_promote_with_no_fragments() {
		let mut scope_stack = ScopeStack::<
			Range<usize>,
			Vec<Range<usize>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>
		>::new();

		scope_stack.promote_last_fragment();
	}

	use crate::scope_stack::Relationship;

	#[test]
	fn can_promote_simplex_fragment() {
		let concept = 0..6;
		let fragment = Fragment::new_simplex(
			concept.clone(),
			VecDeque::<Node<Range<usize>, Vec<Range<usize>>>>::new());
		let mut scope_stack = ScopeStack::new();

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let mut scope = VecDeque::with_capacity(1);
			let node = Node::Simplex(concept.clone(), VecDeque::new());
			scope.push_back(node);
			scopes.push(scope);
			scopes
		};

		scope_stack.fragments.push(fragment);
		scope_stack.promote_last_fragment();

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}

	#[test]
	fn can_promote_complex_fragment() {
		let concept = 6..10;
		let fragment = Fragment::new_complex(
			concept.clone(),
			VecDeque::<Node<Range<usize>, Vec<Range<usize>>>>::new());
		let mut scope_stack = ScopeStack::new();

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let mut scope = VecDeque::with_capacity(1);
			let node = Node::Complex(concept.clone(), VecDeque::new(), VecDeque::new());
			scope.push_back(node);
			scopes.push(scope);
			scopes
		};

		scope_stack.fragments.push(fragment);
		scope_stack.scopes.push(VecDeque::new());
		scope_stack.promote_last_fragment();

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
