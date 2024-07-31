use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

pub trait Marker {}

#[derive(Debug)]
pub struct Node<TNode, TMarker: Marker> {
    node: TNode,
    _marker: PhantomData<TMarker>,
    // This is kinda cursed, but only accessed from typesafe functions
    annotations: HashMap<TypeId, Box<dyn Any>>,
}

impl<TNode, TMarker: Marker> From<TNode> for Node<TNode, TMarker> {
    fn from(value: TNode) -> Self {
        Node::new(value)
    }
}

impl<TNode, TMarker: Marker> From<TNode> for Box<Node<TNode, TMarker>> {
    fn from(value: TNode) -> Self {
        Node::from(value).into()
    }
}

impl<TNode, TMarker: Marker> Node<TNode, TMarker> {
    pub fn new(value: TNode) -> Self {
        Node {
            node: value,
            annotations: HashMap::new(),
            _marker: PhantomData,
        }
    }

    pub fn add_annotation<U: Annotation>(&mut self, annotation: U) {
        self.annotations
            .insert(TypeId::of::<U>(), Box::new(annotation));
    }

    pub fn get_annotation<U: Annotation>(&self) -> Option<&U> {
        self.annotations
            .get(&TypeId::of::<U>())
            .and_then(|boxed| boxed.downcast_ref::<U>())
    }
}

pub trait Annotation: 'static {}

#[derive(Debug)]
pub struct DefaultMarker;

impl Marker for DefaultMarker {}
