use alloc::vec::Vec;
use crate::Node;

/// Contains the fragments used for parsing.
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub enum Fragment<'a> {
	Simplex(&'a [u8], Vec<Node<'a>>),
	Complex(&'a [u8], Vec<Node<'a>>)
}

impl<'a> Fragment<'a> {
	pub fn attach(&mut self, node: Node<'a>) {
		match self {
			Fragment::Simplex(_, attached_nodes) | Fragment::Complex(_, attached_nodes) => {
				attached_nodes.push(node);
			}
		}
	}
}

#[cfg(test)]
mod t {
	use alloc::vec::Vec;
	use super::{Node, Fragment};

	#[test]
	fn can_attach() {
		let concept_name = b"Attaching attachers to simplex fragments";
		let label = b"a";
		let content = b"bcd";
		let mut fragment = Fragment::Simplex(&concept_name[..], Vec::new());
		let attacher = Node::Attacher(&label[..], &content[..]);
		let expected_fragment = Fragment::Simplex(&concept_name[..], {
			let mut attachers = Vec::new();
			attachers.push(Node::Attacher(&label[..], &content[..]));
			attachers
		});

		fragment.attach(attacher);

		assert_eq!(fragment, expected_fragment);
	}
}
