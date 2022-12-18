use crate::native::PhantomData;

/// Contains the fragments used for parsing.
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum Fragment<T, U, V> {
	Simplex(T, V, PhantomData<U>),
	Complex(T, V, PhantomData<U>)
}

use crate::abstracts::{AbstractBoundary, AbstractAttacherNode, AbstractAttacherCollection};

impl<T, U, V> Fragment<T, U, V>
where
	T: AbstractBoundary<usize>,
	U: AbstractAttacherNode,
	V: AbstractAttacherCollection<U> {
	pub fn new_simplex(boundary: T, collection: V) -> Self {
		Fragment::Simplex(boundary, collection, PhantomData)
	}

	pub fn new_complex(boundary: T, collection: V) -> Self {
		Fragment::Complex(boundary, collection, PhantomData)
	}

	pub fn attach(&mut self, node: U) {
		match self {
			Fragment::Simplex(_, attached_nodes, _) | Fragment::Complex(_, attached_nodes, _) => {
				attached_nodes.attach(node);
			}
		}
	}
}

// #[cfg(test)]
// mod t {
// 	use crate::native::Vec;
// 	use super::{Node, Fragment};

// 	#[test]
// 	fn can_attach() {
// 		let concept_name = b"Attaching attachers to simplex fragments";
// 		let label = b"a";
// 		let content = b"bcd";
// 		let mut fragment = Fragment::Simplex(&concept_name[..], Vec::new());
// 		let attacher = Node::Attacher(&label[..], &content[..]);
// 		let expected_fragment = Fragment::Simplex(&concept_name[..], {
// 			let mut attachers = Vec::new();
// 			attachers.push(Node::Attacher(&label[..], &content[..]));
// 			attachers
// 		});

// 		fragment.attach(attacher);

// 		assert_eq!(fragment, expected_fragment);
// 	}
// }
