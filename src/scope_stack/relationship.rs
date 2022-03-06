/// Indicates the relationship of latest node created to the latest fragment.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Relationship {
	Attached,
	Contained
}
