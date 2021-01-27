use crate::{
	nodes::{NodeIndex, NodeMap},
	traits::{Environment, HashT},
};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub(crate) struct Vertex<H: HashT> {
	creator: NodeIndex,
	parents: NodeMap<Option<H>>,
	hash: H,
}

impl<H: HashT> Vertex<H> {
	pub(crate) fn new(creator: NodeIndex, hash: H, parents: NodeMap<Option<H>>) -> Self {
		Vertex {
			creator,
			hash,
			parents,
		}
	}
	pub(crate) fn creator(&self) -> NodeIndex {
		self.creator
	}
	pub(crate) fn hash(&self) -> H {
		self.hash.clone()
	}
}

pub(crate) struct Dag<E: Environment + 'static> {
	vertex_by_hash: HashMap<E::Hash, Vertex<E::Hash>>,
}

impl<E: Environment> Dag<E> {
	pub(crate) fn new() -> Dag<E> {
		Dag {
			vertex_by_hash: HashMap::new(),
		}
	}

	pub(crate) fn contains_hash(&self, hash: &E::Hash) -> bool {
		self.vertex_by_hash.contains_key(hash)
	}

	pub(crate) fn add_vertex(&mut self, vertex: Vertex<E::Hash>) {
		self.vertex_by_hash.insert(vertex.hash, vertex);
	}
}
