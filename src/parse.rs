mod scope_stack;

use alloc::vec::Vec;
use crate::Node;
use crate::lex::{Token, TokenQueue};
use scope_stack::ScopeStack;

/// Returns a collection of nodes based from the source.
///
/// The source is the first argument which contain an array of bytes or a stream of tokens.
///
/// ## Examples
/// ```
/// use chearmyp_parser::{parse, Node};
/// let source = b"
/// ## A sample source
/// hello
/// 	world
/// 	to: everyone
/// hi universe|
/// ";
///
/// let nodes = parse(&source[..]);
/// assert_eq!(nodes, vec![
/// 	Node::LineComment(b" A sample source"),
/// 	Node::Complex(b"hello", vec![Node::Attacher(b"to", b"everyone")], vec![
/// 		Node::Complex(b"world", Vec::new(), Vec::new())
/// 	]),
/// 	Node::Simplex(b"hi universe", Vec::new())
/// ])
/// ```
pub fn parse<'a, T>(stream: T) -> Vec<Node<'a>>
where T: 'a + Into<TokenQueue<'a>> {
	let stream = stream.into();
	let mut scope_stack = ScopeStack::new();

	for token in stream {
		match token {
			Token::Complex(concept) => scope_stack.append_complex(concept),
			Token::Attacher(label, content) => scope_stack.append_attacher(label, content),
			Token::Simplex(concept) => scope_stack.append_simplex(concept),
			Token::ScopeLevel(level) => scope_stack.minimize_scope_level_by(level),
			Token::LineComment(comment) => scope_stack.append_line_comment(comment),
			Token::BlockComment(comment_lines) => scope_stack.append_block_comment(comment_lines),
			Token::LineOthertongue(othertongue) => scope_stack.append_line_othertongue(othertongue),
			Token::BlockOthertongue(othertongue_lines) => {
				scope_stack.append_block_othertongue(othertongue_lines);
			},
			Token::Block(_) | Token::Empty | Token::Invalid => {
				let reason = "The main parser does not accept block, empty, or invalid tokens.";
				let probable_cause = "The token queue may not be generated properly by the lexer.";
				panic!("{} {}", reason, probable_cause);
			}
		}
	}

	scope_stack.finalize()
}


#[cfg(test)]
mod t {
	use alloc::collections::VecDeque;
	use alloc::vec::Vec;
	use super::{Node, Token, TokenQueue};
	use super::parse;

	#[test]
	fn can_parse_short_stream() {
		let mut sample = VecDeque::new();
		sample.push_back(Token::Complex(b"a"));
		let sample = TokenQueue(sample);
		let nodes = parse(sample);
		let mut expected_nodes = Vec::new();
		expected_nodes.push(Node::Complex(b"a", Vec::new(), Vec::new()));
		assert_eq!(nodes, expected_nodes)
	}

	#[test]
	fn can_parse_long_stream() {
		let mut sample = VecDeque::new();
		sample.push_back(Token::Complex(b"b"));
		sample.push_back(Token::ScopeLevel(1));
		sample.push_back(Token::Complex(b"cd"));
		sample.push_back(Token::Complex(b"ef"));
		sample.push_back(Token::ScopeLevel(0));
		sample.push_back(Token::Complex(b"g"));
		let sample = TokenQueue(sample);
		let nodes = parse(sample);

		let mut expected_nodes = Vec::new();
		expected_nodes.push(Node::Complex(b"b", Vec::new(), {
			let mut content = Vec::new();
			content.push(Node::Complex(b"cd", Vec::new(), Vec::new()));
			content.push(Node::Complex(b"ef", Vec::new(), Vec::new()));
			content
		}));
		expected_nodes.push(Node::Complex(b"g", Vec::new(), Vec::new()));

		assert_eq!(nodes, expected_nodes)
	}
}
