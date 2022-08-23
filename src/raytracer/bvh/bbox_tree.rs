use super::aabb::Aabb;
use crate::{
    bvh::bbox_tree::constructor::construct_tree,
    core::Ray,
    geometry::hittable::{Geometry, HitRecord},
};

mod constructor;

#[derive(Clone)]
enum NodePointer {
    Branch { lhs: usize, rhs: usize },
    Leaf(usize),
}

#[derive(Clone)]
pub struct TreeNode {
    bbox: Aabb,
    ptr: NodePointer,
}

pub struct BboxTree<T> {
    root: Option<usize>,
    tree: Vec<TreeNode>,
    leaves: Vec<T>,
}

impl<T> Default for BboxTree<T> {
    fn default() -> Self {
        Self {
            root: None,
            tree: Default::default(),
            leaves: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct BboxTreeWorkspace {
    stack: Vec<usize>,
}

impl<T: Geometry> BboxTree<T> {
    pub fn new(items: Vec<T>) -> BboxTree<T> {
        construct_tree(items)
    }

    pub fn len(&self) -> usize {
        self.leaves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn hit_workspace(
        &self,
        workspace: &mut BboxTreeWorkspace,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<(&T, HitRecord)> {
        let root_idx = self.root?;

        workspace.stack.truncate(0);
        workspace.stack.push(root_idx);

        let mut closest: Option<(&T, HitRecord)> = None;

        while let Some(node_idx) = workspace.stack.pop() {
            let t_closest = closest.as_ref().map(|(_, r)| r.t).unwrap_or(t_max);

            let node = &self.tree[node_idx];
            if !node.bbox.hit2(ray, t_min, t_closest) {
                continue;
            }
            match node.ptr {
                NodePointer::Branch { lhs, rhs } => {
                    workspace.stack.push(lhs);
                    workspace.stack.push(rhs);
                }
                NodePointer::Leaf(idx) => {
                    let obj = &self.leaves[idx];
                    if let Some(hit) = obj.hit(ray, t_min, t_closest) {
                        closest = Some((obj, hit))
                    }
                }
            }
        }
        closest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{Point, Vec3},
        geometry::sphere::Sphere,
    };

    #[test]
    fn size_of_tree_node() {
        // we want this small to fit in cache. if this grows, there should be a good reason.
        // NOTE: its too big right now
        assert_eq!(std::mem::size_of::<TreeNode>(), 72);
    }

    #[test]
    fn emptybbox() {
        let empty_tree = BboxTree::<Sphere>::new(vec![]);

        let mut stack = BboxTreeWorkspace::default();
        let r = Ray::new(Point(Vec3::default()), Vec3::default());
        assert_eq!(
            empty_tree.hit_workspace(&mut stack, &r, 0., std::f64::MAX),
            None
        );
    }

    #[test]
    fn miss_single_obj() {
        let bbox = BboxTree::<Sphere>::new(vec![Sphere {
            center: Point(Vec3::new(0.0, 0.0, -10.0)),
            radius: 0.5,
        }]);

        let mut stack = BboxTreeWorkspace::default();
        let r = Ray::new(Point(Vec3::default()), Vec3::new(1., 0., 0.));
        assert_eq!(bbox.hit_workspace(&mut stack, &r, 0., std::f64::MAX), None);
    }

    #[test]
    fn hit_single_obj() {
        let spheres = vec![Sphere {
            center: Point(Vec3::new(0.0, 0.0, -10.0)),
            radius: 0.5,
        }];
        let bbox = BboxTree::<Sphere>::new(spheres.clone());

        let mut stack = BboxTreeWorkspace::default();
        let r = Ray::new(Point(Vec3::default()), Vec3::new(0., 0., -1.));

        let hit_result = bbox.hit_workspace(&mut stack, &r, 0., std::f64::MAX);
        assert!(hit_result.is_some());
        let (obj, _) = hit_result.unwrap();
        assert_eq!(obj, &spheres[0]);
    }
    #[test]
    fn hit_box_but_not_obj() {
        let spheres = vec![Sphere {
            center: Point(Vec3::new(0.0, 0.0, -2.0)),
            radius: 1.0,
        }];
        let bbox = BboxTree::<Sphere>::new(spheres.clone());

        let mut stack = BboxTreeWorkspace::default();
        let r = Ray::new(Point(Vec3::default()), Vec3::new(0.9, 0.9, -1.5));

        // Verify that we will hit the box
        assert!(
            spheres[0]
                .bounding_box()
                .unwrap()
                .hit2(&r, 0.0, std::f64::MAX),
            "bad test setup, did not hit bounding box"
        );

        let hit_result = bbox.hit_workspace(&mut stack, &r, 0., std::f64::MAX);
        assert!(hit_result.is_none());
    }

    #[test]
    fn hit_first_sphere_in_chain() {
        let first = Sphere {
            center: Point(Vec3::new(0.0, 0.0, -2.0)),
            radius: 1.0,
        };
        let mut spheres = vec![first.clone()];
        for idx in 2..=100 {
            spheres.push(Sphere {
                center: Point(Vec3::new(0.0, 0.0, -2.0 * idx as f64)),
                radius: 1.0,
            });
        }
        let bbox = BboxTree::<Sphere>::new(spheres);

        let mut stack = BboxTreeWorkspace::default();
        let r = Ray::new(Point(Vec3::default()), Vec3::new(0., 0., -1.));

        let hit_result = bbox.hit_workspace(&mut stack, &r, 0., std::f64::MAX);
        assert!(hit_result.is_some());
        let (obj, _) = hit_result.unwrap();
        assert_eq!(obj, &first);
    }

    #[test]
    fn hit_obj_behind_first_box() {
        let spheres = vec![
            Sphere {
                center: Point(Vec3::new(0.0, 0.0, -2.0)),
                radius: 1.0,
            },
            Sphere {
                center: Point(Vec3::new(2.0, 2.0, -4.0)),
                radius: 1.0,
            },
        ];
        let bbox = BboxTree::<Sphere>::new(spheres.clone());

        let mut stack = BboxTreeWorkspace::default();
        let r = Ray::new(Point(Vec3::default()), Vec3::new(0.9, 0.9, -1.5));

        // Verify that we will hit the box
        assert!(
            spheres[0]
                .bounding_box()
                .unwrap()
                .hit2(&r, 0.0, std::f64::MAX),
            "bad test setup, did not hit bounding box"
        );

        let hit_result = bbox.hit_workspace(&mut stack, &r, 0., std::f64::MAX);
        assert!(hit_result.is_some());
        let (obj, _) = hit_result.unwrap();
        assert_eq!(obj, &spheres[1]);
    }

    // TODO test inside obj
    // TODO test inside box but not inside obj
    //
    // TODO test max t is not far enough
    // TODO test min t is too far
}
