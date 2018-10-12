use std::borrow::Cow;

use crate::attr::{AttrDiffer, AttrList, AttrVisitor};
use crate::path::Path;

pub trait NodeVisitor {
    fn on_tag<T>(&mut self, tag: &T)
    where
        T: Tag;

    fn on_text<T>(&mut self, text: &T)
    where
        T: Text;
}

pub trait NodeDiffer {
    fn on_tag<T>(&mut self, curr: &T, ancestor: &T)
    where
        T: Tag;

    fn on_text<T>(&mut self, curr: &T, ancestor: &T)
    where
        T: Text;
}

pub trait Node {
    fn visit<V>(&self, visitor: &mut V)
    where
        V: NodeVisitor;

    fn diff<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: NodeDiffer;
}

pub trait Tag {
    fn is_tag_static(&self) -> bool;

    fn tag(&self) -> &str;

    fn visit_children<V>(&self, visitor: &mut V)
    where
        V: NodeVisitor;

    fn diff_children<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: NodeDiffer;

    fn visit_attr<V>(&self, visitor: &mut V)
    where
        V: AttrVisitor;

    fn diff_attr<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: AttrDiffer;
}

pub struct TagStatic<C, A> {
    tag: &'static str,
    children: C,
    attrs: A,
}

impl<C, A> Tag for TagStatic<C, A>
where
    C: NodeList,
    A: AttrList,
{
    #[inline]
    fn is_tag_static(&self) -> bool {
        true
    }

    #[inline]
    fn tag(&self) -> &str {
        self.tag
    }

    #[inline]
    fn visit_children<V>(&self, visitor: &mut V)
    where
        V: NodeVisitor,
    {
        self.children.visit(visitor);
    }

    #[inline]
    fn diff_children<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: NodeDiffer,
    {
        self.children.diff(&ancestor.children, differ);
    }

    #[inline]
    fn visit_attr<V>(&self, visitor: &mut V)
    where
        V: AttrVisitor,
    {
        self.attrs.visit(visitor);
    }

    #[inline]
    fn diff_attr<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: AttrDiffer,
    {
        self.attrs.diff(&ancestor.attrs, differ);
    }
}

impl<C, A> Node for TagStatic<C, A>
where
    C: NodeList,
    A: AttrList,
{
    #[inline]
    fn visit<V>(&self, visitor: &mut V)
    where
        V: NodeVisitor,
    {
        visitor.on_tag(self);
    }

    #[inline]
    fn diff<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: NodeDiffer,
    {
        debug_assert_eq!(self.tag, ancestor.tag);

        differ.on_tag(self, ancestor);
    }
}

pub struct TagDyn<C, A> {
    tag: Cow<'static, str>,
    children: C,
    attrs: A,
}

impl<C, A> Tag for TagDyn<C, A>
where
    C: NodeList,
    A: AttrList,
{
    #[inline]
    fn is_tag_static(&self) -> bool {
        false
    }

    #[inline]
    fn tag(&self) -> &str {
        self.tag.as_ref()
    }

    #[inline]
    fn visit_children<V>(&self, visitor: &mut V)
    where
        V: NodeVisitor,
    {
        self.children.visit(visitor);
    }

    #[inline]
    fn diff_children<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: NodeDiffer,
    {
        self.children.diff(&ancestor.children, differ);
    }

    #[inline]
    fn visit_attr<V>(&self, visitor: &mut V)
    where
        V: AttrVisitor,
    {
        self.attrs.visit(visitor);
    }

    #[inline]
    fn diff_attr<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: AttrDiffer,
    {
        self.attrs.diff(&ancestor.attrs, differ);
    }
}

impl<C, A> Node for TagDyn<C, A>
where
    C: NodeList,
    A: AttrList,
{
    #[inline]
    fn visit<V>(&self, visitor: &mut V)
    where
        V: NodeVisitor,
    {
        visitor.on_tag(self);
    }

    #[inline]
    fn diff<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: NodeDiffer,
    {
        differ.on_tag(self, ancestor);
    }
}

pub trait Text {
    fn is_static(&self) -> bool;

    fn get(&self) -> &str;
}

pub struct TextStatic(&'static str);

impl Text for TextStatic {
    #[inline]
    fn is_static(&self) -> bool {
        true
    }

    #[inline]
    fn get(&self) -> &str {
        &self.0
    }
}

impl Node for TextStatic {
    #[inline]
    fn visit<V>(&self, visitor: &mut V)
    where
        V: NodeVisitor,
    {
        visitor.on_text(self);
    }

    #[inline]
    fn diff<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: NodeDiffer,
    {
        debug_assert_eq!(self.0, ancestor.0);

        differ.on_text(self, ancestor);
    }
}

pub struct TextDyn(Cow<'static, str>);

impl Text for TextDyn {
    #[inline]
    fn is_static(&self) -> bool {
        false
    }

    #[inline]
    fn get(&self) -> &str {
        self.0.as_ref()
    }
}

impl Node for TextDyn {
    #[inline]
    fn visit<V>(&self, visitor: &mut V)
    where
        V: NodeVisitor,
    {
        visitor.on_text(self);
    }

    #[inline]
    fn diff<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: NodeDiffer,
    {
        differ.on_text(self, ancestor);
    }
}

pub trait NodeList {
    fn visit<V>(&self, visitor: &mut V)
    where
        V: NodeVisitor;

    fn diff<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: NodeDiffer;
}

impl<L1, L2> NodeList for (L1, L2)
where
    L1: NodeList,
    L2: NodeList,
{
    fn visit<V>(&self, visitor: &mut V)
    where
        V: NodeVisitor,
    {
        self.0.visit(visitor);
        self.1.visit(visitor);
    }

    fn diff<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: NodeDiffer,
    {
        self.0.diff(&ancestor.0, differ);
        self.1.diff(&ancestor.1, differ);
    }
}

pub struct NodeListEntry<N>(N);

impl<N> NodeList for NodeListEntry<N>
where
    N: Node,
{
    fn visit<V>(&self, visitor: &mut V)
    where
        V: NodeVisitor,
    {
        self.0.visit(visitor);
    }

    fn diff<D>(&self, ancestor: &Self, differ: &mut D)
    where
        D: NodeDiffer,
    {
        self.0.diff(&ancestor.0, differ);
    }
}
