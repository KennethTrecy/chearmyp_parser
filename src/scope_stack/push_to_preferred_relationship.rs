use crate::abstracts::{
	AbstractBoundary,
	AbstractBoundaryCollection,
	AbstractNode,
	AbstractNodeQueue,
	AbstractAttacherCollection,
	AbstractAttacherNode
};
use crate::node_kind::NodeKind;
use crate::scope_stack::Relationship;
use super::ScopeStack;

impl<T, U, V, W, X, Y> ScopeStack<T, U, V, W, X, Y>
where
	T: AbstractBoundary<usize>,
	U: AbstractBoundaryCollection<usize, T>,
	V: AbstractAttacherNode + From<X>,
	W: AbstractAttacherCollection<V>,
	X: AbstractNode<usize, T, usize, T, U, V, W, X, Y>,
	Y: AbstractNodeQueue<X> {
	pub fn push_to_preferred_relationship(&mut self, node: X) {
		match node.kind() {
			NodeKind::Complex
			| NodeKind::Simplex
			| NodeKind::LineOthertongue
			| NodeKind::BlockOthertongue => {
				self.push_to_last_scope(node);
				self.last_relationship = Relationship::Contained;
			},
			NodeKind::Attacher => {
				if let Some(last_fragment) = self.fragments.last_mut() {
					last_fragment.attach(node.into());
					self.last_relationship = Relationship::Attached;
				} else {
					self.push_to_last_scope(node);
					self.last_relationship = Relationship::Contained;
				}
			},
			NodeKind::LineComment | NodeKind::BlockComment => {
				match self.last_relationship {
					Relationship::Contained => self.push_to_last_scope(node),
					Relationship::Attached => {
						let last_fragment = self.fragments.last_mut().unwrap();
						last_fragment.attach(node.into());
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod t {
	#[cfg(feature = "no_std")]
	use alloc::vec;
	use crate::native::{Range, Vec, VecDeque};
	use crate::node::Node;
	use crate::scope_stack::Fragment;
	use super::{Relationship, ScopeStack};

	#[test]
	fn can_push_as_contained_node() {
		let node = Node::<Range<usize>, Vec<Range<usize>>>::Complex(
			0..5,
			VecDeque::new(),
			VecDeque::new());

		let mut scope_stack = ScopeStack::new();

		let expected_last_scope = {
			let mut scope = VecDeque::new();
			let node = Node::Complex(0..5, VecDeque::new(), VecDeque::new());
			scope.push_back(node);
			scope
		};
		let expected_scopes = {
			let mut scopes = Vec::new();
			scopes.push(expected_last_scope);
			scopes
		};

		scope_stack.push_to_preferred_relationship(node);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}

	#[test]
	fn can_push_attacher_as_attached_node() {
		let concept = 0..3;
		let label = 3..5;
		let content = 5..6;
		let node = Node::<Range<usize>, Vec<Range<usize>>>::Attacher(
			label.clone(),
			content.clone(),
			Vec::new()
		);

		let mut scope_stack = ScopeStack::new();
		let initial_fragment = Fragment::new_simplex(concept.clone(), VecDeque::new());
		scope_stack.fragments.push(initial_fragment);

		let expected_fragment = Fragment::new_simplex(concept.clone(), {
			let mut attached_nodes = VecDeque::new();
			let attacher = Node::Attacher(label.clone(), content.clone(), Vec::new());
			attached_nodes.push_back(attacher);
			attached_nodes
		});
		let expected_fragments = {
			let mut fragments = Vec::new();
			fragments.push(expected_fragment);
			fragments
		};

		scope_stack.push_to_preferred_relationship(node);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Attached);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, {
			let mut scopes = Vec::with_capacity(1);
			scopes.push(VecDeque::new());
			scopes
		});
	}

	#[test]
	fn can_push_attacher_as_contained_node() {
		let label = 3..5;
		let content = 5..6;
		let node = Node::<Range<usize>, Vec<Range<usize>>>::Attacher(
			label.clone(),
			content.clone(),
			Vec::new()
		);

		let mut scope_stack = ScopeStack::new();

		let expected_fragments = Vec::new();

		scope_stack.push_to_preferred_relationship(node);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, {
			let mut scopes = Vec::with_capacity(1);
			scopes.push(vec![ Node::<Range<usize>, Vec<Range<usize>>>::Attacher(
				label.clone(),
				content.clone(),
				Vec::new()
			) ]);
			scopes
		});
	}

	#[test]
	fn can_push_comment_as_contained_node() {
		let comment = 0..4;
		let node = Node::<Range<usize>, Vec<Range<usize>>>::LineComment(comment.clone());

		let mut scope_stack = ScopeStack::new();

		let expected_last_scope = {
			let mut scope = VecDeque::new();
			let node = Node::<Range<usize>, Vec<Range<usize>>>::LineComment(comment.clone());
			scope.push_back(node);
			scope
		};
		let expected_scopes = {
			let mut scopes = Vec::new();
			scopes.push(expected_last_scope);
			scopes
		};

		scope_stack.push_to_preferred_relationship(node);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}

	#[test]
	fn can_push_comment_as_attached_node() {
		let concept = 0..3;
		let comment = 3..6;
		let node = Node::<Range<usize>, Vec<Range<usize>>>::LineComment(comment.clone());

		let mut scope_stack = ScopeStack::new();
		let initial_fragment = Fragment::new_simplex(concept.clone(), VecDeque::new());
		scope_stack.fragments.push(initial_fragment);
		scope_stack.last_relationship = Relationship::Attached;

		let expected_fragment = Fragment::new_simplex(concept.clone(), {
			let mut attached_nodes = VecDeque::new();
			let line_comment = Node::LineComment(comment.clone());
			attached_nodes.push_back(line_comment);
			attached_nodes
		});
		let expected_fragments = {
			let mut fragments = Vec::new();
			fragments.push(expected_fragment);
			fragments
		};

		scope_stack.push_to_preferred_relationship(node);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Attached);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, {
			let mut scopes = Vec::with_capacity(1);
			scopes.push(VecDeque::new());
			scopes
		});
	}
}
