use alloc::vec::Vec;
use crate::parse::scope_stack::Fragment;
use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	/// Appends a simplex fragment to the collection of fragments.
	pub fn append_simplex(&mut self, concept: &'a [u8]) {
		self.necessarily_promote_last_fragments();

		let simplex_fragment = Fragment::Simplex(concept, Vec::new());
		self.fragments.push(simplex_fragment);
	}
}

#[cfg(test)]
mod t {
	use crate::parse::scope_stack::Relationship;
	use super::{Fragment, ScopeStack, Vec};

	#[test]
	pub fn can_append() {
		let concept = b"ab";
		let mut scope_stack = ScopeStack::new();

		let expected_fragments = {
			let mut fragments = Vec::with_capacity(1);
			let fragment = Fragment::Simplex(&concept[..], Vec::new());
			fragments.push(fragment);
			fragments
		};

		scope_stack.append_simplex(&concept[..]);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, {
			let mut scopes = Vec::with_capacity(1);
			let scope = Vec::new();
			scopes.push(scope);
			scopes
		});
	}

	use crate::Node;

	#[test]
	pub fn can_promote_preceding_concepts_first() {
		let first_concept = b"cdef";
		let second_concept = b"gh";
		let mut scope_stack = ScopeStack::new();

		let expected_fragments = {
			let mut fragments = Vec::with_capacity(1);
			let fragment = Fragment::Simplex(&second_concept[..], Vec::new());
			fragments.push(fragment);
			fragments
		};

		let expected_first_scope = {
			let mut scope = Vec::with_capacity(1);
			let node = Node::Simplex(&first_concept[..], Vec::new());
			scope.push(node);
			scope
		};

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			scopes.push(expected_first_scope);
			scopes
		};

		scope_stack.append_simplex(&first_concept[..]);
		scope_stack.append_simplex(&second_concept[..]);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
