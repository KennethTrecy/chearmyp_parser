use crate::parse::scope_stack::Relationship;
use crate::Node;
use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	/// Appends a line comment to the last scope.
	pub fn append_line_comment(&mut self, comment: &'a [u8]) {
		if let Relationship::Contained = self.last_relationship {
			self.necessarily_promote_last_fragments();
		}

		let node = Node::LineComment(comment);
		self.push_to_preferred_relationship(node);
	}
}

#[cfg(test)]
mod t {
	use alloc::vec::Vec;
	use super::{Node, ScopeStack, Relationship};

	#[test]
	pub fn can_append() {
		let comment = b"abcdefghi";
		let mut scope_stack = ScopeStack::new();

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let scope = {
				let mut scope = Vec::new();
				let line_comment = Node::LineComment(&comment[..]);
				scope.push(line_comment);
				scope
			};
			scopes.push(scope);
			scopes
		};

		scope_stack.append_line_comment(&comment[..]);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
