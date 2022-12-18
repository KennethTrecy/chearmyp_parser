mod fragment;
mod relationship;

use crate::native::{Vec, PhantomData};
use fragment::Fragment;
use relationship::Relationship;

pub struct ScopeStack<T, U, V, W, X, Y> {
	level: usize,
	last_relationship: Relationship,
	fragments: Vec<Fragment<T, V, W>>,
	scopes: Vec<Y>,
	_abstract_boundary: PhantomData<T>,
	_abstract_boundary_collection: PhantomData<U>,
	_abstract_attacher_node: PhantomData<V>,
	_abstract_attacher_collection: PhantomData<W>,
	_abstract_node: PhantomData<X>,
	_abstract_node_queue: PhantomData<Y>
}

// These modules are arranged according to preferred refactoring sequence.
mod push_to_last_scope;
mod push_to_preferred_relationship;
mod promote_last_fragment;
mod necessarily_promote_last_fragments;
mod minimize_scope_level_by;
mod finalize;
mod appenders;

use crate::abstracts::{
	AbstractBoundary,
	AbstractBoundaryCollection,
	AbstractNode,
	AbstractNodeQueue,
	AbstractAttacherCollection,
	AbstractAttacherNode
};

impl<T, U, V, W, X, Y> ScopeStack<T, U, V, W, X, Y>
where
	T: AbstractBoundary<usize>,
	U: AbstractBoundaryCollection<usize, T>,
	V: AbstractAttacherNode,
	W: AbstractAttacherCollection<V>,
	X: AbstractNode<usize, T, usize, T, U, V, W, X, Y>,
	Y: AbstractNodeQueue<X> {
	/// Creates a scope stack that serves as the memory for the main parser.
	pub fn new() -> Self {
		let level = 0;
		let last_relationship = Relationship::Contained;
		let fragments = Vec::new();
		let mut scopes = Vec::with_capacity(1);

		scopes.push(Y::new());

		Self {
			level,
			last_relationship,
			fragments,
			scopes,
			_abstract_boundary: PhantomData,
			_abstract_boundary_collection: PhantomData,
			_abstract_attacher_node: PhantomData,
			_abstract_attacher_collection: PhantomData,
			_abstract_node: PhantomData,
			_abstract_node_queue: PhantomData
		}
	}
}
