use super::ScopeStack;

impl<'a> ScopeStack<'a> {
	/// Promotes some fragments until a specified scope level has been reached.
	///
	/// The scope level must be equal to the number of fragments that exist.
	pub fn minimize_scope_level_by(&mut self, minimum_level: usize) {
		self.level = minimum_level;
		self.necessarily_promote_last_fragments();
	}
}

#[cfg(test)]
mod t {
	use alloc::vec::Vec;
	use crate::parse::scope_stack::Relationship;
	use super::ScopeStack;

	#[test]
	fn can_minimize_with_no_fragments() {
		let mut scope_stack = ScopeStack::new();

		scope_stack.minimize_scope_level_by(0);

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

	use crate::Node;
	use crate::parse::scope_stack::Fragment;

	#[test]
	fn can_minimize_multiple_levels() {
		let complex = b"abcdefgh";
		let simplex = b"ijkl";
		let complex_fragment = Fragment::Complex(&complex[..], Vec::new());
		let simplex_fragment = Fragment::Simplex(&simplex[..], Vec::new());
		let mut scope_stack = ScopeStack::new();

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let mut scope = Vec::with_capacity(1);
			let node = Node::Complex(&complex[..], Vec::new(), {
				let mut contents = Vec::new();
				let simplex = Node::Simplex(&simplex[..], Vec::new());
				contents.push(simplex);
				contents
			});
			scope.push(node);
			scopes.push(scope);
			scopes
		};

		scope_stack.fragments.push(complex_fragment);
		scope_stack.scopes.push(Vec::new());
		scope_stack.minimize_scope_level_by(1);
		scope_stack.fragments.push(simplex_fragment);
		scope_stack.minimize_scope_level_by(0);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}

	#[test]
	fn can_necessarily_reduce_separately() {
		let complex = b"mn";
		let simplex = b"opq";
		let complex_fragment = Fragment::Complex(&complex[..], Vec::new());
		let simplex_fragment = Fragment::Simplex(&simplex[..], Vec::new());
		let mut scope_stack = ScopeStack::new();

		let expected_scopes = {
			let mut scopes = Vec::with_capacity(1);
			let mut scope = Vec::with_capacity(2);
			let complex = Node::Complex(&complex[..], Vec::new(), Vec::new());
			scope.push(complex);

			let simplex = Node::Simplex(&simplex[..], Vec::new());
			scope.push(simplex);
			scopes.push(scope);
			scopes
		};

		scope_stack.fragments.push(complex_fragment);
		scope_stack.scopes.push(Vec::new());
		scope_stack.minimize_scope_level_by(0);
		scope_stack.fragments.push(simplex_fragment);
		scope_stack.minimize_scope_level_by(0);

		assert_eq!(scope_stack.level, 0);
		assert_eq!(scope_stack.last_relationship, Relationship::Contained);
		assert_eq!(scope_stack.fragments, Vec::new());
		assert_eq!(scope_stack.scopes, expected_scopes);
	}
}
