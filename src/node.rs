use alloc::vec::Vec;

/// Contains the nodes used for parsing.
#[derive(Debug, PartialEq)]
pub enum Node<'a> {
	LineComment(&'a [u8]),
	BlockComment(Vec<&'a [u8]>),
	Simplex(&'a [u8], Vec<Node<'a>>),
	Complex(&'a [u8], Vec<Node<'a>>, Vec<Node<'a>>),
	Attacher(&'a [u8], &'a [u8]),
	LineOthertongue(&'a [u8]),
	BlockOthertongue(Vec<&'a [u8]>)
}
