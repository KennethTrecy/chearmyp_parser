use alloc::vec::Vec;
use crate::Node;
use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	/// Appends a block othertongue to the last scope.
	pub fn append_block_othertongue(&mut self, othertongue_lines: Vec<&'a [u8]>) {
		self.necessarily_promote_last_fragments();

		let node = Node::BlockOthertongue(othertongue_lines);
		self.push_to_preferred_relationship(node);
	}
}

#[cfg(test)]
mod t {
	use crate::parse::scope_stack::Relationship;
	use super::{Node, ScopeStack, Vec};

	#[test]
	pub fn can_append() {
		let first_othertongue_line = b"abcdefghijkl";
		let second_othertongue_line = b"mnop";
		let othertongue_lines = {
			let mut lines = Vec::new();
			lines.push(&first_othertongue_line[..]);
			lines.push(&second_othertongue_line[..]);
			lines
		};

		let mut scope_stack = ScopeStack::new();

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let scope = {
				let mut scope = Vec::new();
				let line_othertongue = Node::BlockOthertongue(othertongue_lines.clone());
				scope.push(line_othertongue);
				scope
			};
			scopes.push(scope);
			scopes
		};

		scope_stack.append_block_othertongue(othertongue_lines);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
