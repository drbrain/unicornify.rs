use crate::Axis;
use crate::Vector;

use crate::unicornify::Thing;

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;
use std::hash::Hasher;
use std::iter::Iterator;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    inner: Rc<RefCell<Thing>>,
}

impl Node {
    fn new(thing: Thing) -> Self {
        let inner = Rc::new(RefCell::new(thing));

        Node { inner }
    }

    pub fn rotate_around(&self, other: Vector, angle: f64, axis: Axis) {
        let thing = self.inner.borrow_mut();

        let new_thing = thing.rotate_around(other, angle, axis);

        match new_thing {
            Some(t) => {
                self.inner.replace(t);
            }
            None => (),
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.borrow().hash(state);
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Figure {
    pub nodes: Vec<Node>,
}

impl Figure {
    pub fn new() -> Figure {
        let nodes = Vec::new();

        Figure { nodes }
    }

    pub fn push(&mut self, thing: Thing) {
        self.nodes.push(Node::new(thing));
    }

    pub fn iter(&self) -> FigureIterator {
        FigureIterator::new(self)
    }
}

pub struct FigureIterator {
    seen: HashMap<Node, bool>,
    todo: VecDeque<Node>,
}

impl FigureIterator {
    pub fn new(figure: &Figure) -> Self {
        let seen = HashMap::new();
        let mut todo = VecDeque::with_capacity(figure.nodes.len());

        for node in &figure.nodes {
            todo.push_back((*node).clone());
        }

        FigureIterator { seen, todo }
    }
}

impl Iterator for FigureIterator {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        // TODO while we haven't seen an item in todo
        let next = loop {
            let front = match self.todo.pop_front() {
                None => {
                    return None;
                }
                Some(t) => t,
            };

            if self.seen.contains_key(&front) {
                continue;
            }

            break front;
        };

        match *next.inner.borrow() {
            Thing::BallT(ref _b) => {}
            Thing::BoneT(ref b) => {
                self.todo.push_back(Node::new(Thing::BallT(b.b1.clone())));
                self.todo.push_back(Node::new(Thing::BallT(b.b2.clone())));
            }
            Thing::FigureT(ref f) => {
                for n in &f.nodes {
                    self.todo.push_back((*n).clone());
                }
            }
            Thing::SteakT(ref s) => {
                self.todo.push_back(Node::new(Thing::BallT(s.b1.clone())));
                self.todo.push_back(Node::new(Thing::BallT(s.b2.clone())));
                self.todo.push_back(Node::new(Thing::BallT(s.b3.clone())));
            }
        };

        Some(next)
    }
}
