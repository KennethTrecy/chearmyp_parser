use crate::Node;
use crate::parse::scope_stack::Relationship;
use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	pub fn push_to_preferred_relationship(&mut self, node: Node<'a>) {
		match node {
			Node::Complex(_, _, _) | Node::Simplex(_, _) | Node::LineOthertongue(_)
			| Node::BlockOthertongue(_) => {
				self.push_to_last_scope(node);
				self.last_relationship = Relationship::Contained;
			},
			Node::Attacher(_, _) => {
				let last_fragment = self.fragments.last_mut().unwrap();
				last_fragment.attach(node);
				self.last_relationship = Relationship::Attached;
			},
			Node::LineComment(_) | Node::BlockComment(_) => {
				match self.last_relationship {
					Relationship::Contained => self.push_to_last_scope(node),
					Relationship::Attached => {
						let last_fragment = self.fragments.last_mut().unwrap();
						last_fragment.attach(node);
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod t {
	use alloc::vec::Vec;
	use crate::parse::scope_stack::Fragment;
	use super::{Node, Relationship, ScopeStack};

	#[test]
	fn can_push_as_contained_node() {
		let concept = b"abcde";
		let node = Node::Complex(&concept[..], Vec::new(), Vec::new());

		let mut scope_stack = ScopeStack::new();

		let expected_last_scope = {
			let mut scope = Vec::new();
			let node = Node::Complex(&concept[..], Vec::new(), Vec::new());
			scope.push(node);
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
	fn can_push_as_attached_node() {
		let concept = b"fgh";
		let label = b"ij";
		let content = b"k";
		let node = Node::Attacher(&label[..], &content[..]);

		let mut scope_stack = ScopeStack::new();
		let initial_fragment = Fragment::Simplex(&concept[..], Vec::new());
		scope_stack.fragments.push(initial_fragment);

		let expected_fragment = Fragment::Simplex(&concept[..], {
			let mut attached_nodes = Vec::new();
			let attacher = Node::Attacher(&label[..], &content[..]);
			attached_nodes.push(attacher);
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
			scopes.push(Vec::new());
			scopes
		});
	}

	#[test]
	fn can_push_comment_as_contained_node() {
		let comment = b"lmno";
		let node = Node::LineComment(&comment[..]);

		let mut scope_stack = ScopeStack::new();

		let expected_last_scope = {
			let mut scope = Vec::new();
			let node = Node::LineComment(&comment[..]);
			scope.push(node);
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
		let concept = b"pqr";
		let comment = b"stu";
		let node = Node::LineComment(&comment[..]);

		let mut scope_stack = ScopeStack::new();
		let initial_fragment = Fragment::Simplex(&concept[..], Vec::new());
		scope_stack.fragments.push(initial_fragment);
		scope_stack.last_relationship = Relationship::Attached;

		let expected_fragment = Fragment::Simplex(&concept[..], {
			let mut attached_nodes = Vec::new();
			let line_comment = Node::LineComment(&comment[..]);
			attached_nodes.push(line_comment);
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
			scopes.push(Vec::new());
			scopes
		});
	}
}
