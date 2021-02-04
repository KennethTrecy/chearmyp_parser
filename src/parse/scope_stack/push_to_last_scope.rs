use crate::Node;
use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	pub fn push_to_last_scope(&mut self, node: Node<'a>) {
		let last_scope = self.scopes.last_mut().unwrap();
		last_scope.push(node);
	}
}

#[cfg(test)]
mod t {
	use alloc::vec::Vec;
	use crate::parse::scope_stack::Relationship;
	use super::{Node, ScopeStack};

	#[test]
	fn can_push_to_last_scope() {
		let concept = b"abcd";
		let node =  Node::Complex(&concept[..], Vec::new(), Vec::new());

		let mut scope_stack = ScopeStack::new();

		let mut expected_scopes = Vec::new();
		let expected_last_scope = {
			let mut scope = Vec::new();
			let expected_complex = Node::Complex(&concept[..], Vec::new(), Vec::new());
			scope.push(expected_complex);
			scope
		};
		expected_scopes.push(expected_last_scope);

		scope_stack.push_to_last_scope(node);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
