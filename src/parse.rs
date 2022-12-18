use crate::abstracts::{
	AbstractBoundary,
	AbstractBoundaryCollection,

	AbstractToken,
	AbstractTokenQueue,
	AbstractComplexToken,
	AbstractSimplexToken,
	AbstractAttacherToken,
	AbstractScopeLevelToken,
	AbstractLineCommentToken,
	AbstractBlockCommentToken,
	AbstractLineOthertongueToken,
	AbstractBlockOthertongueToken,

	AbstractNode,
	AbstractNodeQueue,
	AbstractAttacherCollection,
	AbstractAttacherNode
};
use crate::token_kind::TokenKind;
use crate::ScopeStack;

/// Returns a collection of nodes based from the source.
///
/// The source is the first argument which contain an array of bytes or a stream of tokens.
///
/// ## Examples
/// ```
/// use std::cmp::PartialEq;
/// use std::ops::Range;
/// use std::collections::VecDeque;
/// use chearmyp_token::Token;
/// use chearmyp_lexer::lex;
/// use chearmyp_node::Node;
/// use chearmyp_parser::parse;
/// let source = b"
/// ## A sample source
/// hello
/// 	world
/// 		to: everyone
/// hi universe|
/// ";
///
/// let tokens = lex(&&source[..], VecDeque::new());
///
/// println!("{:?}", &tokens);
///
/// type DefaultToken = Token<Range<usize>, Vec<Range<usize>>>;
/// type AttacherToken = DefaultToken;
/// type ScopeLevelToken = DefaultToken;
/// type ComplexToken = DefaultToken;
/// type SimplexToken = DefaultToken;
/// type LineCommentToken = DefaultToken;
/// type BlockCommentToken = DefaultToken;
/// type LineOthertongueToken = DefaultToken;
/// type BlockOthertongueToken = DefaultToken;
///
/// let nodes = parse::<
/// 	_, _, _, _, _, _, _,
/// 	VecDeque<Node<
/// 		Range<usize>,
/// 		Vec<Range<usize>>
/// 	>>,
/// 	AttacherToken,
/// 	ScopeLevelToken,
/// 	ComplexToken,
/// 	SimplexToken,
/// 	LineCommentToken,
/// 	BlockCommentToken,
/// 	LineOthertongueToken,
/// 	BlockOthertongueToken
/// >(tokens);
/// assert_eq!(nodes, VecDeque::from(vec![
/// 	Node::LineComment(2..18),
/// 	Node::Complex(
/// 		19..24,
/// 		VecDeque::new(),
/// 		VecDeque::from(vec![
/// 			Node::Complex(
/// 				26..31,
/// 				VecDeque::from(vec![Node::Attacher(34..36, 38..46, vec![0..0])]),
/// 				VecDeque::new()
/// 			)
/// 		])
/// 	),
/// 	Node::Simplex(47..58, VecDeque::new())
/// ]))
/// ```
pub fn parse<T, U, V, W, X, Y, Z, A, B, C, D, E, F, G, H, I>(mut tokens: W) -> A
where
	T: AbstractBoundary<usize>,
	U: AbstractBoundaryCollection<usize, T>,
	V: AbstractToken<usize, T, usize, T, U>,
	W: AbstractTokenQueue<usize, T, usize, T, U, V>,
	X: AbstractAttacherNode + From<Z>,
	Y: AbstractAttacherCollection<X>,
	Z: AbstractNode<usize, T, usize, T, U, X, Y, Z, A>,
	A: AbstractNodeQueue<Z>,
	B: AbstractAttacherToken<Label = T, Content = T> + From<V>,
	C: AbstractScopeLevelToken + From<V>,
	D: AbstractComplexToken<Complex = T> + From<V>,
	E: AbstractSimplexToken<Simplex = T> + From<V>,
	F: AbstractLineCommentToken<Line = T> + From<V>,
	G: AbstractBlockCommentToken<Block = U> + From<V>,
	H: AbstractLineOthertongueToken<Line = T> + From<V>,
	I: AbstractBlockOthertongueToken<Block = U> + From<V> {
	let mut scope_stack = ScopeStack::<T, U, X, Y, Z, A>::new();

	loop {
		let token = tokens.shift_token();

		match token {
			Some(token) => match token.kind() {
				TokenKind::Complex => {
					let concept = D::from(token).consume();
					scope_stack.append_complex(concept)
				},
				TokenKind::Attacher => {
					let (label, content) = B::from(token).consume();
					scope_stack.append_attacher(label, content)
				},
				TokenKind::Simplex => {
					let concept = E::from(token).consume();
					scope_stack.append_simplex(concept)
				},
				TokenKind::ScopeLevel => {
					let level = C::from(token).level();
					scope_stack.minimize_scope_level_by(level)
				},
				TokenKind::LineComment => {
					let line = F::from(token).consume();
					scope_stack.append_line_comment(line)
				},
				TokenKind::BlockComment => {
					let block = G::from(token).consume();
					scope_stack.append_block_comment(block)
				},
				TokenKind::LineOthertongue => {
					let line = H::from(token).consume();
					scope_stack.append_line_othertongue(line)
				},
				TokenKind::BlockOthertongue => {
					let block = I::from(token).consume();
					scope_stack.append_block_othertongue(block);
				}
			},
			None => break
		}
	}

	scope_stack.finalize()
}


#[cfg(test)]
mod t {
	use crate::native::{Range, Vec, VecDeque};
	use crate::token::Token;
	use crate::node::Node;
	use super::parse;

	type DefaultToken = Token<Range<usize>, Vec<Range<usize>>>;
	type AttacherToken = DefaultToken;
	type ScopeLevelToken = DefaultToken;
	type ComplexToken = DefaultToken;
	type SimplexToken = DefaultToken;
	type LineCommentToken = DefaultToken;
	type BlockCommentToken = DefaultToken;
	type LineOthertongueToken = DefaultToken;
	type BlockOthertongueToken = DefaultToken;

	#[test]
	fn can_parse_short_stream() {
		let mut sample_queue = VecDeque::new();
		sample_queue.push_back(Token::Complex(0..1));
		let nodes = parse::<
			_, _, _, _, _, _, _,
			VecDeque<Node<
				Range<usize>,
				Vec<Range<usize>>
			>>,
			AttacherToken,
			ScopeLevelToken,
			ComplexToken,
			SimplexToken,
			LineCommentToken,
			BlockCommentToken,
			LineOthertongueToken,
			BlockOthertongueToken
		>(sample_queue);
		let mut expected_nodes = Vec::new();
		expected_nodes.push(Node::Complex(0..1, VecDeque::new(), VecDeque::new()));
		assert_eq!(nodes, expected_nodes)
	}

	#[test]
	fn can_parse_long_stream() {
		let mut sample_queue = VecDeque::new();
		sample_queue.push_back(Token::Complex(1..2));
		sample_queue.push_back(Token::ScopeLevel(1));
		sample_queue.push_back(Token::Complex(2..4));
		sample_queue.push_back(Token::Complex(4..6));
		sample_queue.push_back(Token::ScopeLevel(0));
		sample_queue.push_back(Token::Complex(6..7));
		let nodes = parse::<
			_, _, _, _, _, _, _,
			VecDeque<Node<
				Range<usize>,
				Vec<Range<usize>>
			>>,
			AttacherToken,
			ScopeLevelToken,
			ComplexToken,
			SimplexToken,
			LineCommentToken,
			BlockCommentToken,
			LineOthertongueToken,
			BlockOthertongueToken
		>(sample_queue);

		let mut expected_nodes = Vec::new();
		expected_nodes.push(Node::Complex(1..2, VecDeque::new(), {
			let mut content = VecDeque::new();
			content.push_back(Node::Complex(2..4, VecDeque::new(), VecDeque::new()));
			content.push_back(Node::Complex(4..6, VecDeque::new(), VecDeque::new()));
			content
		}));
		expected_nodes.push(Node::Complex(6..7, VecDeque::new(), VecDeque::new()));

		assert_eq!(nodes, expected_nodes)
	}
}
