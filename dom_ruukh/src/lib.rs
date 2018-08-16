//! The Virtual DOM library which backs the `ruukh` frontend framework.

use velement::VElement;
use vtext::VText;

mod vtext;
mod velement;

/// A virtual node in a virtual DOM tree.
#[derive(Debug)]
pub enum VNode {
    Text(VText),
    Element(VElement),
}
