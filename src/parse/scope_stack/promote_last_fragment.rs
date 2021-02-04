use crate::Node;
use crate::parse::scope_stack::Fragment;
use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	/// Promotes the last fragment into a node in the last scope.
	pub fn promote_last_fragment(&mut self) {
		let last_fragment = self.fragments.pop().unwrap();
		let node;

		match last_fragment {
			Fragment::Simplex(simplex_name, attachers) => {
				node = Node::Simplex(simplex_name, attachers);
			},
			Fragment::Complex(complex_name, attachers) => {
				let last_scope = self.scopes.pop().unwrap();
				node = Node::Complex(complex_name, attachers, last_scope);
			}
		}

		self.push_to_preferred_relationship(node);
	}
}

#[cfg(test)]
mod t {
	use super::ScopeStack;

	#[test]
	#[should_panic]
	fn cannot_promote_with_no_fragments() {
		let mut scope_stack = ScopeStack::new();

		scope_stack.promote_last_fragment();
	}

	use alloc::vec::Vec;
	use crate::Node;
	use crate::parse::scope_stack::Relationship;
	use super::Fragment;

	#[test]
	fn can_promote_simplex_fragment() {
		let concept = b"abcdef";
		let fragment = Fragment::Simplex(&concept[..], Vec::new());
		let mut scope_stack = ScopeStack::new();

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let mut scope = Vec::with_capacity(1);
			let node = Node::Simplex(&concept[..], Vec::new());
			scope.push(node);
			scopes.push(scope);
			scopes
		};

		scope_stack.fragments.push(fragment);
		scope_stack.promote_last_fragment();

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}

	#[test]
	fn can_promote_complex_fragment() {
		let concept = b"ghij";
		let fragment = Fragment::Complex(&concept[..], Vec::new());
		let mut scope_stack = ScopeStack::new();

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let mut scope = Vec::with_capacity(1);
			let node = Node::Complex(&concept[..], Vec::new(), Vec::new());
			scope.push(node);
			scopes.push(scope);
			scopes
		};

		scope_stack.fragments.push(fragment);
		scope_stack.scopes.push(Vec::new());
		scope_stack.promote_last_fragment();

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
