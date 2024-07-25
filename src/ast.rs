use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct Node<T> {
    node: T,
    // This is kinda cursed, but only accessed from typesafe functions
    annotations: HashMap<TypeId, Box<dyn Any>>,
}

impl<T> Node<T> {
    fn from(node: T) -> Self {
        Node {
            node,
            annotations: HashMap::new(),
        }
    }

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
