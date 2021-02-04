use crate::Node;
use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	/// Appends an attacher to the collection of attached nodes in last fragment.
	pub fn append_attacher(&mut self, label: &'a [u8], content: &'a [u8]) {
		let node = Node::Attacher(label, content);
		self.push_to_preferred_relationship(node);
	}
}

#[cfg(test)]
mod t {
	use alloc::vec::Vec;
	use crate::parse::scope_stack::{Fragment, Relationship};
	use super::{Node, ScopeStack};

	#[test]
	pub fn can_append() {
		let concept = b"abcdefgh";
		let label = b"ij";
		let content = b"k";
		let mut scope_stack = ScopeStack::new();

		let expected_fragments = {
			let mut fragments = Vec::with_capacity(1);
			let fragment = Fragment::Simplex(&concept[..], {
				let mut attachers = Vec::new();
				let attacher = Node::Attacher(&label[..], &content[..]);
				attachers.push(attacher);
				attachers
			});
			fragments.push(fragment);
			fragments
		};

		scope_stack.append_simplex(&concept[..]);
		scope_stack.append_attacher(&label[..], &content[..]);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Attached);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, {
			let mut scopes = Vec::with_capacity(1);
			let scope = Vec::new();
			scopes.push(scope);
			scopes
		});
	}
}
