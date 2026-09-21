use crate::algo::util::TagCollector;
use crate::{Graph, Node, Tag};
use recz_adt::{Map, Set};

pub(super) struct TagCleaner<'a> {
    in_graph: &'a Graph,
    out_graph: &'a Graph,
    ingoing_open_tags: Map<Node<'a>, Set<Tag>>,
    outgoing_close_tags: Map<Node<'a>, Set<Tag>>,
    // visited: Set<Node<'a>>,
    visited: Map<Node<'a>, Set<u64>>,
    tag_collector: TagCollector,
}

impl<'a> TagCleaner<'a> {
    pub(super) fn new(in_graph: &'a Graph, out_graph: &'a Graph) -> Self {
        Self {
            in_graph,
            out_graph,
            ingoing_open_tags: Map::default(),
            outgoing_close_tags: Map::default(),
            visited: Map::default(),
            tag_collector: TagCollector::new(),
        }
    }

    pub(super) fn run(&mut self) {
        if self.in_graph.is_empty() {
            return;
        }
        self.recurse_ingoing_open_tags(self.in_graph.start_node());
        self.recurse_outgoing_close_tags(self.in_graph.nodes().last().unwrap());
        self.build_cleaned_graph();
    }

    fn build_cleaned_graph(&self) {
        assert!(self.out_graph.is_empty());

        for group in self.in_graph.groups() {
            self.out_graph.group(group.label().clone());
        }

        let conv_map = Map::default();

        for in_node in self.in_graph.nodes() {
            let out_node = self.out_graph.node();
            out_node.set_kind(in_node.kind());
            conv_map.insert(in_node, out_node);
        }

        for in_edge in self.in_graph.edges() {
            let out_source = conv_map[&in_edge.source()];
            let out_target = conv_map[&in_edge.target()];
            let out_edge = out_source.connect(out_target);
            out_edge.add_symbols(in_edge.symbols());
            for tag in in_edge.tags() {
                match tag {
                    Tag::OpenGroup(id) => {
                        if let Some(common_tags) = self.ingoing_open_tags.get(&in_edge.source())
                            && common_tags.contains(&Tag::OpenGroup(id))
                        // && !in_edge.contains_tag(Tag::CloseGroup(id))
                        {
                            continue;
                        }
                        out_edge.add_tag(Tag::OpenGroup(id));
                    }
                    Tag::CloseGroup(id) => {
                        if let Some(common_tags) = self.outgoing_close_tags.get(&in_edge.target())
                            && common_tags.contains(&Tag::CloseGroup(id))
                        // && !in_edge.contains_tag(Tag::OpenGroup(id))
                        {
                            continue;
                        }
                        out_edge.add_tag(Tag::CloseGroup(id));
                    }
                    tag => {
                        out_edge.add_tag(tag);
                    }
                }
            }
        }
    }

    fn recurse_ingoing_open_tags(&mut self, node: Node<'a>) {
        self.collect_open_tags(node);

        let new_check = self.tag_collector.checksum();
        let passed_checks = self.visited.entry(node).or_default();
        if passed_checks.contains(&new_check) {
            return;
        }
        passed_checks.insert(new_check);

        for (edge, target) in node.targets() {
            self.tag_collector.extend(edge.tags());
            self.recurse_ingoing_open_tags(target);
            self.tag_collector.shorten(edge.tags().rev());
        }
    }

    fn collect_open_tags(&mut self, node: Node<'a>) {
        if self.tag_collector.is_empty() {
            return;
        }

        if let Some(common_tags) = self.ingoing_open_tags.get_mut(&node) {
            let mut tags_to_remove = Vec::default();
            for tag in common_tags.iter() {
                if !self.tag_collector.contains(tag) {
                    tags_to_remove.push(*tag);
                }
            }
            for tag in tags_to_remove {
                common_tags.remove(&tag);
            }
        } else {
            self.ingoing_open_tags.insert(
                node,
                self.tag_collector
                    .tags()
                    .filter(|t| matches!(t, Tag::OpenGroup(_)))
                    .collect(),
            );
        }
    }

    fn recurse_outgoing_close_tags(&mut self, node: Node<'a>) {
        self.collect_close_tags(node);

        let new_check = self.tag_collector.checksum();
        let passed_checks = self.visited.entry(node).or_default();
        if passed_checks.contains(&new_check) {
            return;
        }
        passed_checks.insert(new_check);

        for (source, edge) in node.sources() {
            self.tag_collector.extend(edge.tags());
            self.recurse_outgoing_close_tags(source);
            self.tag_collector.shorten(edge.tags().rev());
        }
    }

    fn collect_close_tags(&mut self, node: Node<'a>) {
        if self.tag_collector.is_empty() {
            return;
        }

        if let Some(common_tags) = self.outgoing_close_tags.get_mut(&node) {
            let mut tags_to_remove = Vec::default();
            for tag in common_tags.iter() {
                if !self.tag_collector.contains(tag) {
                    tags_to_remove.push(*tag);
                }
            }
            for tag in tags_to_remove {
                common_tags.remove(&tag);
            }
        } else {
            self.outgoing_close_tags.insert(
                node,
                self.tag_collector
                    .tags()
                    .filter(|t| matches!(t, Tag::CloseGroup(_)))
                    .collect(),
            );
        }
    }
}
