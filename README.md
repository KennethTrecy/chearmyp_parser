# Chearmyp
This is an experimental, general purpose, human-readable, language. In this context, general purpose
means that it can be used as a markup, programming, command, and more.

## Syntax Overview
There are five general kinds of nodes that exist in Chearmyp. Take note that Chearmyp still have
unstable syntax.
1. *Comment*. These nodes may be used for documentation purposes. It has two subkinds: *line*
	comments and *block* comments.
	- *Line* comment. These are comments that only exist in one line and have a pound sign (`#`)
		prefix.
		```
		# This is a line comment.
		```
	- *Block* comment. These comments may have one or more lines. They have three pound signs (`###`)
		as their prefix and suffix.
		```
		###
		This is a block comment.
		###

		# Below is an indented block comment.
			###
				This is an indented block comment.
		### These pound signs will not terminate the comment.
				### This one too.
			Only the pound signs below will end the block comment.
			###
		```

2. *Simplex*. These nodes can be thought of as basic concepts that other concepts can use or
	include. For example, `letter`. Since these are simple, they cannot contain other concepts.
	Simplexes must end using a vertical line (`|`). It means that the concept "ends" there.
	```
	letter|	# This is an example of simplex. Use tabs to comment.
	1|	# A simplex can be a number.
	?|	# A simplex can be anything as long it does not start a pound sign.
	example city|	# And they may contain spaces too!
	```

3. *Complex*. These nodes are counterpart of *simplexes*. These are concepts that can contain other
	*simplexes* and *complexes*. For example `word`, it can contain the `letter|`. To express the
	containment of other nodes, those nodes must be indented using a tab.
	```
	# `word`, `punctuation`, and `binary` are examples of complex
	word
		letters
			letter|
			letter|
		part_of_speech|
		definition|

	punctuation
		.|
		?|
		!|

	binary
		0|
		1|
	```

4. *Attacher*. These nodes are pair of concepts attached to a *simplex* or *complex*. These
	concepts may be a metadata, adjective, reference, and others. They are written below or right
	side of the *simplex* or *complex* they are attaching to. *Attachers* must have a colon (`:`) and
	followed tab(s) or spaces(s) between the pairs namely label and content.
	```
	books
		book_a|
			price: $1			# Attaches book a's price.
			title: Title A		# Attaches book a's title.
			author: Author A	# Attaches book a's author.

		book_b|
			price:	$2			# Attaches book b's price.
			title:	Title B	# Attaches book b's title.
			author:	Author B	# Attaches book b's author.

		# Attachers can be written in one line.
		book_c|
			price: $3	title: Title C	author: Author C

		name: Book set A	# Attaches book set's name.
	```

5. *Othertongue*. These nodes contain a different language from the rest of the document. Their
	syntax is similar to *comments*. Due to the variety of languages, there are concepts also, that
	have no direct translation to the current concept. However, *othertongues* must be contained to a
	*complex*.
	- *Line* othertongue. These othertoungues are similar to *line* comments. Their containment must
		be expressed by indention using tab(s) or in one line after the *complex*.
		```
		numbers
			0|
				= zero
			1|
				= one
				= ...

		attacher
			= a pair of concepts that attach to other complex or simplex
		```
	- *Block* othertongue. These othertongues are similar to *block* comments. Instead of pound
		signs, they are identified using three equal signs(`===`).
		```
		languages
			natural
				English
					===
					Good morning!
					===
				Filipino
					===
					Magandang umaga!
					===
				Kapampangan
					===
					Mayap na abak!
					===
			programming
				===
				print "Good morning!"
				===
		```

## Origin
Some parts of the repository was based from [`filled_bare_metal`] branch of [Feo Template].

## Parser
This repository contains a parser library for Chearmyp which represents the source as syntax tree.

## Lexer
The lexer has been forked. Visit the [repository of the lexer] for more details.

### Author
Chearmyp Parser was created by Kenneth Trecy Tobias.

[`filled_bare_metal`]: https://github.com/KennethTrecy/feo_template/tree/filled_bare_metal
[Feo Template]: https://github.com/KennethTrecy/feo_template
[repository of the lexer]: https://github.com/KennethTrecy/chearmyp_lexer
