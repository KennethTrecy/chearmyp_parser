use alloc::vec::Vec;
use crate::parse::scope_stack::Fragment;
use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	/// Appends a complex fragment to the collection of fragments.
	pub fn append_complex(&mut self, concept: &'a [u8]) {
		self.necessarily_promote_last_fragments();

		let complex_fragment = Fragment::Complex(concept, Vec::new());
		self.fragments.push(complex_fragment);
		self.scopes.push(Vec::new());
	}
}

#[cfg(test)]
mod t {
	use crate::parse::scope_stack::Relationship;
	use super::{Fragment, ScopeStack, Vec};

	#[test]
	pub fn can_append() {
		let concept = b"abc";
		let mut scope_stack = ScopeStack::new();

		let expected_fragments = {
			let mut fragments = Vec::with_capacity(1);
			let fragment = Fragment::Complex(&concept[..], Vec::new());
			fragments.push(fragment);
			fragments
		};

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let scope = Vec::new();
			scopes.push(scope);

			let complex_scope = Vec::new();
			scopes.push(complex_scope);

			scopes
		};

		scope_stack.append_complex(&concept[..]);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, expected_scopes);
	}

	use crate::Node;

	#[test]
	pub fn can_promote_preceding_concepts_first() {
		let first_concept = b"def";
		let second_concept = b"fg";
		let mut scope_stack = ScopeStack::new();

		let expected_fragments = {
			let mut fragments = Vec::with_capacity(1);
			let fragment = Fragment::Complex(&second_concept[..], Vec::new());
			fragments.push(fragment);
			fragments
		};

		let expected_first_scope = {
			let mut scope = Vec::with_capacity(1);
			let node = Node::Complex(&first_concept[..], Vec::new(), Vec::new());
			scope.push(node);
			scope
		};

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(2);
			scopes.push(expected_first_scope);

			let complex_scope = Vec::new();
			scopes.push(complex_scope);

			scopes
		};

		scope_stack.append_complex(&first_concept[..]);
		scope_stack.append_complex(&second_concept[..]);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
