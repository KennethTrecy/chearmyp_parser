use crate::Node;
use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	/// Appends a line othertongue to the last scope.
	pub fn append_line_othertongue(&mut self, othertongue: &'a [u8]) {
		self.necessarily_promote_last_fragments();

		let node = Node::LineOthertongue(othertongue);
		self.push_to_preferred_relationship(node);
	}
}

#[cfg(test)]
mod t {
	use alloc::vec::Vec;
	use crate::parse::scope_stack::Relationship;
	use super::{Node, ScopeStack};

	#[test]
	pub fn can_append() {
		let othertongue = b"abcdefghijkl";
		let mut scope_stack = ScopeStack::new();

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let scope = {
				let mut scope = Vec::new();
				let line_othertongue = Node::LineOthertongue(&othertongue[..]);
				scope.push(line_othertongue);
				scope
			};
			scopes.push(scope);
			scopes
		};

		scope_stack.append_line_othertongue(&othertongue[..]);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
