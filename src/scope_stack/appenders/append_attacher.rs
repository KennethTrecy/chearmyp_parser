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
	/// Appends an attacher to the collection of attached nodes in last fragment.
	pub fn append_attacher(&mut self, label: T, content: T) {
		self.necessarily_promote_last_fragments();

		let node = X::new_attacher(label, content, U::new(0, 0));
		self.push_to_preferred_relationship(node);
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec, VecDeque};
	use crate::node::Node;
	use crate::scope_stack::{Fragment, Relationship};
	use super::ScopeStack;

	#[test]
	pub fn can_append() {
		let concept = 0..8;
		let label = 8..10;
		let content = 10..11;
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
			let fragment = Fragment::new_simplex(concept.clone(), {
				let mut attachers = VecDeque::new();
				let attacher = Node::Attacher(label.clone(), content.clone(), {
					let mut comments = Vec::new();
					comments.push(0..0);
					comments
				});
				attachers.push_back(attacher);
				attachers
			});
			fragments.push(fragment);
			fragments
		};

		scope_stack.append_simplex(concept.clone());
		scope_stack.minimize_scope_level_by(1);
		scope_stack.append_attacher(label.clone(), content.clone());

		assert_eq!(scope_stack.level, 1);
		assert_eq!(scope_stack.last_relationship, Relationship::Attached);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, {
			let mut scopes = Vec::with_capacity(1);
			let scope = VecDeque::new();
			scopes.push(scope);
			scopes
		});
	}

	#[test]
	pub fn can_append_after_a_simplex() {
		let simplex_concept = 11..15;
		let complex_concept = 15..19;
		let label = 19..21;
		let content = 21..22;
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
			let fragment = Fragment::new_complex(complex_concept.clone(), {
				let mut attachers = VecDeque::new();
				let attacher = Node::Attacher(label.clone(), content.clone(), {
					let mut comments = Vec::new();
					comments.push(0..0);
					comments
				});
				attachers.push_back(attacher);
				attachers
			});
			fragments.push(fragment);
			fragments
		};

		scope_stack.append_complex(complex_concept.clone());
		scope_stack.minimize_scope_level_by(1);
		scope_stack.append_simplex(simplex_concept.clone());
		scope_stack.append_attacher(label.clone(), content.clone());

		assert_eq!(scope_stack.level, 1);
		assert_eq!(scope_stack.last_relationship, Relationship::Attached);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, {
			let mut scopes = Vec::with_capacity(1);
			let initial_scope = VecDeque::new();
			scopes.push(initial_scope);
			scopes.push({
				let mut scope = VecDeque::new();
				let simplex = Node::Simplex(simplex_concept.clone(), VecDeque::new());
				scope.push_back(simplex);
				scope
			});
			scopes
		});
	}

	#[test]
	pub fn can_append_after_a_complex() {
		let first_complex_concept = 22..23;
		let second_complex_concept = 23..24;
		let label = 24..25;
		let content = 25..26;
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
			let fragment = Fragment::new_complex(first_complex_concept.clone(), {
				let mut attachers = VecDeque::new();
				let attacher = Node::Attacher(label.clone(), content.clone(), {
					let mut comments = Vec::new();
					comments.push(0..0);
					comments
				});
				attachers.push_back(attacher);
				attachers
			});
			fragments.push(fragment);
			fragments
		};

		scope_stack.append_complex(first_complex_concept.clone());
		scope_stack.minimize_scope_level_by(1);
		scope_stack.append_complex(second_complex_concept.clone());
		scope_stack.append_attacher(label.clone(), content.clone());

		assert_eq!(scope_stack.level, 1);
		assert_eq!(scope_stack.last_relationship, Relationship::Attached);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, {
			let mut scopes = Vec::with_capacity(1);
			let initial_scope = VecDeque::new();
			scopes.push(initial_scope);
			scopes.push({
				let mut scope = VecDeque::new();
				let simplex = Node::Complex(
					second_complex_concept.clone(),
					VecDeque::new(),
					VecDeque::new());
				scope.push_back(simplex);
				scope
			});
			scopes
		});
	}
}
