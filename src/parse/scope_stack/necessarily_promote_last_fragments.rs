use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	/// Promotes the last fragments until it matches the scope level.
	pub fn necessarily_promote_last_fragments(&mut self) {
		let minimum_level = self.level;
		let current_level = self.fragments.len();

		for _ in minimum_level..current_level {
			self.promote_last_fragment();
		}
	}
}

#[cfg(test)]
mod t {
	use alloc::vec::Vec;
	use crate::Node;
	use crate::parse::scope_stack::Relationship;
	use super::ScopeStack;

	#[test]
	fn cannot_promote_with_no_fragments() {
		let mut scope_stack = ScopeStack::new();

		scope_stack.necessarily_promote_last_fragments();

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, {
			let mut scopes = Vec::with_capacity(1);
			let scope = Vec::new();
			scopes.push(scope);
			scopes
		});
	}

	use crate::parse::scope_stack::Fragment;

	#[test]
	pub fn can_keep_level_stable() {
		let fragment = b"abcdefg";
		let concept = b"hij";
		let complex = Fragment::Complex(&fragment[..], Vec::new());
		let simplex = Fragment::Simplex(&concept[..], Vec::new());
		let target_level = 1;
		let mut scope_stack = ScopeStack::new();

		let expected_fragments = {
			let mut fragments = Vec::with_capacity(1);
			let fragment = Fragment::Complex(&fragment[..], Vec::new());
			fragments.push(fragment);
			fragments
		};

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(2);
			scopes.push(Vec::new());
			let scope = {
				let mut scope = Vec::new();
				let concept = Node::Simplex(&concept[..], Vec::new());
				scope.push(concept);
				scope
			};
			scopes.push(scope);
			scopes
		};

		scope_stack.fragments.push(complex);
		scope_stack.scopes.push(Vec::new());
		scope_stack.level = target_level;
		scope_stack.fragments.push(simplex);
		scope_stack.necessarily_promote_last_fragments();

		assert_eq!(scope_stack.level, target_level);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, expected_fragments);
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
