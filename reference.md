# Chearmyp Syntax Reference
These document contains specific details about the nodes' syntax in Chearmyp.

## Comment
In general, comments must be denoted with a pound sign (`#`).

### Line comment
Prefix: `#`
Suffix: new line or end-of-file
These comments must within one line only.

### Block comment
Prefix: zero to many tabs then, `###` then new line
Suffix: zero to many tabs (must match number of tabs in prefix) then, `###` then, new line or
			end-of-file.
These comments can have multiple lines. Their contents must be preserved excluding the prefix and
suffix.

## Simplex
Suffix: `|` then, new line or one or many tabs or end-of-file
These basic concepts cannot include other concepts, yet they can attach them.

## Complex
Suffix: new line or one to many tabs or end-of-file
These can contain other kinds of concepts by placing them to the next line with indention using a
tab.

## Attacher
Separator: `:` then, one or many tabs or spaces
Suffix: new line or one or many tabs or end-of-file
These are pair of concepts attached to a *simplex* or *complex*. There are two parts of it:
1. Label
2. Content

Their attachment to *simplex* or *complex* can be expressed by one of the following:
- If the *simplex* or *complex*, and *attacher*, are in one line, they must be separated by tabs.
- Otherwise, they must be on the next line of the *simplex* and *complex* with indented by one tab.

Attachers may also have subsequent attachers separated by tabs. In addition, there should be no two attachers with the same label.

## Othertongue
They are similar to comments but with additional features. They are denoted using equal sign (`=`).
There must be a *complex* that contains them.

### Line Othertongue
Prefix: `=` then, space
Suffix: new line or end-of-file
These must be within one line only just like the *line* comments.

### Block Othertongue
Prefix: zero to many tabs then, `===` then new line
Suffix: zero to many tabs (must match number of tabs in prefix) then, `===` then, new line or
			end-of-file.
It is similar to *block* comments. Therefore, they may contain multiple lines.
