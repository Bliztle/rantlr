use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Debug)]
pub struct Node<T> {
    node: T,
    // This is kinda cursed, but only accessed from typesafe functions
    annotations: HashMap<TypeId, Box<dyn Any>>,
}

impl<T> From<T> for Node<T> {
    fn from(value: T) -> Self {
        Node {
            node: value,
            annotations: HashMap::new(),
        }
    }
}

impl<T> From<T> for Box<Node<T>> {
    fn from(value: T) -> Self {
        Node::from(value).into()
    }
}

impl<T> Node<T> {
    fn add_annotation<U: Annotation>(&mut self, annotation: U) {
        self.annotations
            .insert(TypeId::of::<U>(), Box::new(annotation));
    }

    fn get_annotation<U: Annotation>(&self) -> Option<&U> {
        self.annotations
            .get(&TypeId::of::<U>())
            .and_then(|boxed| boxed.downcast_ref::<U>())
    }
}

pub trait Annotation: 'static {}
