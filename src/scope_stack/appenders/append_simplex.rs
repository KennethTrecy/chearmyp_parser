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
	W: AbstractAttacherCollection<V>,
	X: AbstractNode<usize, T, usize, T, U, V, W, X, Y>,
	Y: AbstractNodeQueue<X> {
	/// Appends a simplex fragment to the collection of fragments.
	pub fn append_simplex(&mut self, concept: T) {
		self.necessarily_promote_last_fragments();

		let simplex_fragment = Fragment::new_simplex(concept, W::new());
		self.fragments.push(simplex_fragment);
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec, VecDeque};
	use crate::node::Node;
	use crate::scope_stack::Relationship;
	use super::{Fragment, ScopeStack};

	#[test]
	pub fn can_append() {
		let concept = 0..2;
		let mut scope_stack = ScopeStack::<
			Range<usize>,
			Vec<Range<usize>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>
		>::new();

		let expected_fragments = {
			let mut fragments = Vec::with_capacity(1);
			let fragment = Fragment::new_simplex(concept.clone(), VecDeque::new());
			fragments.push(fragment);
			fragments
		};

		scope_stack.append_simplex(concept.clone());

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, {
			let mut scopes = Vec::with_capacity(1);
			let scope = VecDeque::new();
			scopes.push(scope);
			scopes
		});
	}

	#[test]
	pub fn can_promote_preceding_concepts_first() {
		let first_concept = 2..6;
		let second_concept = 6..8;
		let mut scope_stack = ScopeStack::<
			Range<usize>,
			Vec<Range<usize>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>
		>::new();

		let expected_fragments = {
			let mut fragments = Vec::with_capacity(1);
			let fragment = Fragment::new_simplex(second_concept.clone(), VecDeque::new());
			fragments.push(fragment);
			fragments
		};

		let expected_first_scope = {
			let mut scope = VecDeque::with_capacity(1);
			let node = Node::Simplex(first_concept.clone(), VecDeque::new());
			scope.push_back(node);
			scope
		};

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			scopes.push(expected_first_scope);
			scopes
		};

		scope_stack.append_simplex(first_concept.clone());
		scope_stack.append_simplex(second_concept.clone());

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
