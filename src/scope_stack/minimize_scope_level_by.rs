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
	/// Promotes some fragments until a specified scope level has been reached.
	///
	/// The scope level must be equal to the number of fragments that exist.
	pub fn minimize_scope_level_by(&mut self, minimum_level: usize) {
		self.level = minimum_level;
		self.necessarily_promote_last_fragments();
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec, VecDeque};
	use crate::node::Node;
	use crate::scope_stack::Relationship;
	use super::ScopeStack;

	#[test]
	fn can_minimize_with_no_fragments() {
		let mut scope_stack = ScopeStack::<
			Range<usize>,
			Vec<Range<usize>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>
		>::new();

		scope_stack.minimize_scope_level_by(0);

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
	fn can_minimize_multiple_levels() {
		let complex = 0..8;
		let simplex = 8..12;
		let complex_fragment = Fragment::new_complex(complex.clone(), VecDeque::new());
		let simplex_fragment = Fragment::new_simplex(simplex.clone(), VecDeque::new());
		let mut scope_stack = ScopeStack::<
			Range<usize>,
			Vec<Range<usize>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>
		>::new();

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let mut scope = VecDeque::with_capacity(1);
			let node = Node::Complex(complex.clone(), VecDeque::new(), {
				let mut contents = VecDeque::new();
				let simplex = Node::Simplex(simplex.clone(), VecDeque::new());
				contents.push_back(simplex);
				contents
			});
			scope.push_back(node);
			scopes.push(scope);
			scopes
		};

		scope_stack.fragments.push(complex_fragment);
		scope_stack.scopes.push(VecDeque::new());
		scope_stack.minimize_scope_level_by(1);
		scope_stack.fragments.push(simplex_fragment);
		scope_stack.minimize_scope_level_by(0);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}

	#[test]
	fn can_necessarily_reduce_separately() {
		let complex = 12..14;
		let simplex = 14..17;
		let complex_fragment = Fragment::new_complex(complex.clone(), VecDeque::new());
		let simplex_fragment = Fragment::new_simplex(simplex.clone(), VecDeque::new());
		let mut scope_stack = ScopeStack::<
			Range<usize>,
			Vec<Range<usize>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>,
			Node<Range<usize>, Vec<Range<usize>>>,
			VecDeque<Node<Range<usize>, Vec<Range<usize>>>>
		>::new();

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let mut scope = VecDeque::with_capacity(2);
			let complex = Node::Complex(complex.clone(), VecDeque::new(), VecDeque::new());
			scope.push_back(complex);

			let simplex = Node::Simplex(simplex.clone(), VecDeque::new());
			scope.push_back(simplex);
			scopes.push(scope);
			scopes
		};

		scope_stack.fragments.push(complex_fragment);
		scope_stack.scopes.push(VecDeque::new());
		scope_stack.minimize_scope_level_by(0);
		scope_stack.fragments.push(simplex_fragment);
		scope_stack.minimize_scope_level_by(0);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
