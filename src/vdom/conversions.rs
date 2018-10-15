//! Conversion from the non-component types to VNode for use in html!
//! expression blocks. Allows the user to use basic types such as string and
//! number types ergonomically within html! expression blocks.

use crate::vdom::{vlist::VList, vtext::VText, VNode};
use std::borrow::Cow;

impl<RCTX> From<String> for VNode<RCTX> {
    fn from(value: String) -> VNode<RCTX> {
        VNode::from(VText::text(value))
    }
}

impl<'a, RCTX> From<&'a String> for VNode<RCTX> {
    fn from(value: &'a String) -> VNode<RCTX> {
        VNode::from(VText::text(value.as_str()))
    }
}

impl<'a, RCTX> From<&'a str> for VNode<RCTX> {
    fn from(value: &'a str) -> VNode<RCTX> {
        VNode::from(VText::text(value))
    }
}

impl<'a, RCTX> From<Cow<'a, str>> for VNode<RCTX> {
    fn from(value: Cow<'a, str>) -> VNode<RCTX> {
        VNode::from(VText::text(value))
    }
}

impl<'a, RCTX> From<&'a Cow<'a, str>> for VNode<RCTX> {
    fn from(value: &'a Cow<'a, str>) -> VNode<RCTX> {
        VNode::from(VText::text(value.as_ref()))
    }
}

macro_rules! impl_with_to_string {
    ($($t:ty),*) => {
        $(
            impl<RCTX> From<$t> for VNode<RCTX> {
                fn from(value: $t) -> VNode<RCTX> {
                    VNode::from(VText::text(value.to_string()))
                }
            }
        )*
    };
}

impl_with_to_string!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool
);

impl<RCTX> From<Vec<VNode<RCTX>>> for VNode<RCTX> {
    fn from(value: Vec<VNode<RCTX>>) -> VNode<RCTX> {
        VNode::from(VList::from(value))
    }
}
