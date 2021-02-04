use alloc::vec::Vec;
use crate::Node;
use crate::parse::scope_stack::Relationship;
use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	/// Appends a block comment to the last scope.
	pub fn append_block_comment(&mut self, comment_lines: Vec<&'a [u8]>) {
		if let Relationship::Contained = self.last_relationship {
			self.necessarily_promote_last_fragments();
		}

		let node = Node::BlockComment(comment_lines);
		self.push_to_preferred_relationship(node);
	}
}

#[cfg(test)]
mod t {
	use super::{Node, ScopeStack, Relationship, Vec};

	#[test]
	pub fn can_append() {
		let comment_line = b"abcdefghijk";
		let comment_lines = {
			let mut lines = Vec::new();
			lines.push(&comment_line[..]);
			lines
		};

		let mut scope_stack = ScopeStack::new();

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let scope = {
				let mut scope = Vec::new();
				let line_comment = Node::BlockComment(comment_lines.clone());
				scope.push(line_comment);
				scope
			};
			scopes.push(scope);
			scopes
		};

		scope_stack.append_block_comment(comment_lines);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
