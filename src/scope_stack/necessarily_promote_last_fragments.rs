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
	/// Promotes the last fragments until it matches the scope level.
	pub fn necessarily_promote_last_fragments(&mut self) {
		let minimum_level = self.level;
		let current_level = self.fragments.len();

		for _ in minimum_level..current_level {
			self.promote_last_fragment();
		}
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec, VecDeque};
	use crate::node::Node;
	use crate::scope_stack::Relationship;
	use super::ScopeStack;

	#[test]
	fn cannot_promote_with_no_fragments() {
		let mut scope_stack = ScopeStack::<
			Range<usize>,
			Vec<Range<usize>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>
		>::new();

		scope_stack.necessarily_promote_last_fragments();

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, {
			let mut scopes = Vec::with_capacity(1);
			let scope = VecDeque::new();
			scopes.push(scope);
			scopes
		});
	}

	use crate::scope_stack::Fragment;

	#[test]
	pub fn can_keep_level_stable() {
		let fragment = 0..7;
		let concept = 7..10;
		let complex = Fragment::new_complex(fragment.clone(), VecDeque::new());
		let simplex = Fragment::new_simplex(concept.clone(), VecDeque::new());
		let target_level = 1;
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
			let fragment = Fragment::new_complex(fragment.clone(), VecDeque::new());
			fragments.push(fragment);
			fragments
		};

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(2);
			scopes.push(VecDeque::new());
			let scope = {
				let mut scope = VecDeque::new();
				let concept = Node::Simplex(concept.clone(), VecDeque::new());
				scope.push_back(concept);
				scope
			};
			scopes.push(scope);
			scopes
		};

		scope_stack.fragments.push(complex);
		scope_stack.scopes.push(VecDeque::new());
		scope_stack.level = target_level;
		scope_stack.fragments.push(simplex);
		scope_stack.necessarily_promote_last_fragments();

		assert_eq!(scope_stack.level, target_level);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
