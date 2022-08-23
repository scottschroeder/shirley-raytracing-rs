use std::collections::HashSet;

use super::{BboxTree, NodePointer, TreeNode};
use crate::{
    bvh::aabb::{bounding, surrounding_box},
    geometry::hittable::Geometry,
};

pub fn construct_tree<T: Geometry>(items: Vec<T>) -> BboxTree<T> {
    if items.is_empty() {
        return BboxTree::default();
    }
    let mut leaf_nodes = Vec::with_capacity(items.len());
    for (idx, value) in items.iter().enumerate() {
        if let Some(bbox) = value.bounding_box() {
            leaf_nodes.push(TreeNode {
                bbox,
                ptr: NodePointer::Leaf(idx),
            })
        }
    }

    let mut tree = Vec::new();
    let ordered_by_dimm = LeafDimmSlices::new(&leaf_nodes);
    let all_nodes = BoxSet {
        inner: (0..leaf_nodes.len()).collect(),
    };
    let root_node = partition_nodes(&mut tree, &leaf_nodes, &all_nodes, &ordered_by_dimm);
    let root = Some(tree.len());
    tree.push(root_node);
    BboxTree {
        root,
        tree,
        leaves: items,
    }
}

#[derive(Default)]
struct BoxSet {
    inner: HashSet<usize>,
}

impl BoxSet {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

fn sorted_with_idx(iter: impl Iterator<Item = f64>) -> Vec<(usize, f64)> {
    let mut buf = iter.enumerate().collect::<Vec<_>>();
    buf.sort_unstable_by(|a, b| a.1.total_cmp(&b.1));
    buf
}

struct LeafDimmSlices {
    x_min: Vec<(usize, f64)>,
    // x_max: Vec<(usize, f64)>,
    y_min: Vec<(usize, f64)>,
    // y_max: Vec<(usize, f64)>,
    z_min: Vec<(usize, f64)>,
    // z_max: Vec<(usize, f64)>,
}

impl LeafDimmSlices {
    fn new(nodes: &[TreeNode]) -> LeafDimmSlices {
        LeafDimmSlices {
            x_min: sorted_with_idx(nodes.iter().map(|tn| tn.bbox.min.0.x())),
            // x_max: sorted_with_idx(nodes.iter().map(|tn| tn.bbox.max.0.x())),
            y_min: sorted_with_idx(nodes.iter().map(|tn| tn.bbox.min.0.y())),
            // y_max: sorted_with_idx(nodes.iter().map(|tn| tn.bbox.max.0.y())),
            z_min: sorted_with_idx(nodes.iter().map(|tn| tn.bbox.min.0.z())),
            // z_max: sorted_with_idx(nodes.iter().map(|tn| tn.bbox.max.0.z())),
        }
    }
}

fn split_median(input: &BoxSet, order: &[(usize, f64)]) -> (BoxSet, BoxSet) {
    let mut lhs = BoxSet::default();
    let mut rhs = BoxSet::default();

    for (idx, _) in order {
        if !input.inner.contains(idx) {
            continue;
        }
        if lhs.inner.len() < input.inner.len() / 2 {
            lhs.inner.insert(*idx);
        } else {
            rhs.inner.insert(*idx);
        }
    }

    (lhs, rhs)
}

fn split_space(input: &BoxSet, order: &[(usize, f64)]) -> (BoxSet, BoxSet) {
    let mut lhs = BoxSet::default();
    let mut rhs = BoxSet::default();

    let avail = order
        .iter()
        .filter(|(idx, _)| input.inner.contains(idx))
        .collect::<Vec<_>>();

    log::trace!("available nodes: {:?}", avail);

    let mid = (avail[avail.len() - 1].1 + avail[0].1) / 2.0;
    log::trace!("midpoint: {}", mid);

    for (loop_idx, (idx, pt)) in avail.iter().enumerate() {
        if loop_idx == 0 {
            lhs.inner.insert(*idx);
            continue;
        }
        if *pt < mid {
            lhs.inner.insert(*idx);
        } else {
            rhs.inner.insert(*idx);
        }
    }

    log::trace!("lhs[{}] rhs[{}]", lhs.len(), rhs.len());

    (lhs, rhs)
}

fn total_area(nodes: &[TreeNode], lhs: &BoxSet, rhs: &BoxSet) -> f64 {
    let lhs_area = bounding(lhs.inner.iter().map(|bidx| &nodes[*bidx].bbox))
        .map(|b| b.area())
        .unwrap_or(0.0);
    let rhs_area = bounding(rhs.inner.iter().map(|bidx| &nodes[*bidx].bbox))
        .map(|b| b.area())
        .unwrap_or(0.0);
    lhs_area + rhs_area
}

fn evaluate_split(nodes: &[TreeNode], sides: &(BoxSet, BoxSet), name: &str) -> f64 {
    let (lhs, rhs) = sides;
    let area = total_area(nodes, lhs, rhs);

    log::debug!(
        "for split `{}`, lhs[{}] rhs[{}], area[{:.2}]",
        name,
        lhs.len(),
        rhs.len(),
        area
    );

    area
}

fn split_best(nodes: &[TreeNode], input: &BoxSet, all_order: &LeafDimmSlices) -> (BoxSet, BoxSet) {
    let mut splits = Vec::new();
    splits.push(("xmin_median", split_median(input, &all_order.x_min)));
    // splits.push(("xmax_median", split_median(input, &all_order.x_max)));
    splits.push(("xmin_space", split_space(input, &all_order.x_min)));
    // splits.push(("xmax_space", split_space(input, &all_order.x_max)));
    splits.push(("ymin_median", split_median(input, &all_order.y_min)));
    // splits.push(("ymax_median", split_median(input, &all_order.y_max)));
    splits.push(("ymin_space", split_space(input, &all_order.y_min)));
    // splits.push(("ymax_space", split_space(input, &all_order.y_max)));
    splits.push(("zmin_median", split_median(input, &all_order.z_min)));
    // splits.push(("zmax_median", split_median(input, &all_order.z_max)));
    splits.push(("zmin_space", split_space(input, &all_order.z_min)));
    // splits.push(("zmax_space", split_space(input, &all_order.z_max)));

    let (_, sides, name) = splits
        .into_iter()
        .map(|(name, sides)| {
            let score = evaluate_split(nodes, &sides, name);
            (score, sides, name)
        })
        .min_by(|(a, _, _), (b, _, _)| a.total_cmp(b))
        .unwrap();

    let worst_frac =
        std::cmp::max(sides.0.len(), sides.1.len()) as f64 / (sides.0.len() + sides.1.len()) as f64;
    log::debug!("selected `{}`, {}", name, worst_frac);

    sides
}

fn partition_nodes(
    tree: &mut Vec<TreeNode>,
    nodes: &[TreeNode],
    working_set: &BoxSet,
    all_order: &LeafDimmSlices,
) -> TreeNode {
    match working_set.inner.len() {
        0 => panic!("trying to partition an empty set"),
        1 => nodes[*working_set.inner.iter().next().unwrap()].clone(),
        _ => {
            let (lhs_set, rhs_set) = split_best(nodes, working_set, all_order);

            let lhs = partition_nodes(tree, nodes, &lhs_set, all_order);
            let rhs = partition_nodes(tree, nodes, &rhs_set, all_order);

            let bbox = surrounding_box(&lhs.bbox, &rhs.bbox);
            let lhs_idx = tree.len();
            tree.push(lhs);
            let rhs_idx = tree.len();
            tree.push(rhs);

            TreeNode {
                bbox,
                ptr: NodePointer::Branch {
                    lhs: lhs_idx,
                    rhs: rhs_idx,
                },
            }
        }
    }
}
