pub mod marked;

pub use marked::Annotation;

pub type Node<TNode> = marked::Node<TNode, marked::DefaultMarker>;
