use crate::abstracts::{
	AbstractBoundary,
	AbstractBoundaryCollection,
	AbstractNode,
	AbstractNodeQueue,
	AbstractAttacherCollection,
	AbstractAttacherNode
};
use crate::scope_stack::Relationship;
use super::ScopeStack;

impl<T, U, V, W, X, Y> ScopeStack<T, U, V, W, X, Y>
where
	T: AbstractBoundary<usize>,
	U: AbstractBoundaryCollection<usize, T>,
	V: AbstractAttacherNode + From<X>,
	W: AbstractAttacherCollection<T, V>,
	X: AbstractNode<usize, T, usize, T, U, T, V, W, X, Y>,
	Y: AbstractNodeQueue<X> {
	/// Appends a line comment to the last scope.
	pub fn append_line_comment(&mut self, comment: T) {
		if let Relationship::Contained = self.last_relationship {
			self.necessarily_promote_last_fragments();
		}

		let node = X::new_line_comment(comment);
		self.push_to_preferred_relationship(node);
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec, VecDeque};
	use crate::node::Node;
	use super::{Relationship, ScopeStack};

	#[test]
	pub fn can_append() {
		let comment = 0..9;
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
			let scope = {
				let mut scope = Vec::new();
				let line_comment = Node::LineComment(comment.clone());
				scope.push(line_comment);
				scope
			};
			scopes.push(scope);
			scopes
		};

		scope_stack.append_line_comment(comment.clone());

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
