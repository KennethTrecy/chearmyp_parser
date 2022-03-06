#![cfg_attr(feature = "no_std", no_std)]

//! # Chearmyp Parser
//! Please read the README.md for more information.
//!
//! ## Features available
//! - `no_std`: Uses the `core` crate instead of `std` crate.

#[cfg(feature = "no_std")]
extern crate alloc;

// #[cfg(test)]
mod native {
	#[cfg(feature = "no_std")]
	pub use core::{
		ops::Range,
		marker::PhantomData
	};

	#[cfg(feature = "no_std")]
	pub use alloc::{
		vec::Vec,
		collections::VecDeque
	};

	#[cfg(not(feature = "no_std"))]
	pub use std::{
		vec::Vec,
		ops::Range,
		marker::PhantomData,
		collections::VecDeque
	};
}

mod abstracts {
	pub use abstract_chearmyp_source::{
		AbstractSource,
		AbstractSourceCollection,
		ComparableAbstractSource
	};

	pub use abstract_chearmyp_boundary::{
		AbstractBoundary,
		AbstractBoundaryCollection
	};

	pub use abstract_chearmyp_token::{
		AbstractToken,
		AbstractTokenQueue,
		AbstractComplexToken,
		AbstractSimplexToken,
		AbstractAttacherToken,
		AbstractScopeLevelToken,
		AbstractLineCommentToken,
		AbstractBlockCommentToken,
		AbstractLineOthertongueToken,
		AbstractBlockOthertongueToken
	};

	#[cfg(test)]
	pub use abstract_chearmyp_token::{
		SimpleAbstractToken
	};

	pub use abstract_chearmyp_node::{
		AbstractNode,
		AbstractNodeQueue,
		AbstractAttacherCollection,
		AbstractAttacherNode
	};
}

mod token_kind {
	pub use abstract_chearmyp_token::TokenKind;
}

mod node_kind {
	pub use abstract_chearmyp_node::NodeKind;
}

#[cfg(test)]
mod token {
	pub use chearmyp_token::Token;
}

#[cfg(test)]
mod node {
	pub use chearmyp_node::Node;
}

mod scope_stack;

/// Contains the parser.
mod parse;

use scope_stack::ScopeStack;
pub use parse::parse;
