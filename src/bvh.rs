use std::{cmp::Ordering, process::exit, sync::Arc};

use rand::Rng;

use crate::{aabb::AABB, hit::Hittable};

struct BvhNode {
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
    bounding_box: AABB,
}

impl BvhNode {
    fn new(
        mut src_objects: Vec<Arc<dyn Hittable>>,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis: i32 = rand::thread_rng().gen_range(0..2);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => Self::box_x_compare,
        };

        let (left, right) = match src_objects.len() {
            1 => {
                let item = src_objects.remove(0);
                (Some(item), None)
            }
            2 if comparator(&src_objects[0], &src_objects[1])
                == Ordering::Greater =>
            {
                (Some(src_objects.remove(0)), Some(src_objects.remove(1)))
            }
            2 => (Some(src_objects.remove(1)), Some(src_objects.remove(2))),
            _ => {
                src_objects.sort_by(comparator);
                let others = src_objects.split_off(src_objects.len() / 2);
                let left: Arc<dyn Hittable> =
                    Arc::new(BvhNode::new(src_objects, time0, time1));
                let right: Arc<dyn Hittable> =
                    Arc::new(BvhNode::new(others, time0, time1));
                (Some(left), Some(right))
            }
        };

        // let elem = match object_span {
        //     1 => object_start,
        //     2 if comparator(object_start, object_start_next) => object_start,
        //     2 => object_start,
        //     _ => {
        //         // objects.sort_by(comparator);

        //         let mid = start + object_span / 2;

        //         // let left: Arc<dyn Hittable> =
        //         // let right: Arc<dyn Hittable> =
        //         //     ;
        //         let x: Arc<dyn Hittable> =
        //             Arc::new(BvhNode::new(src_objects, start, mid, time0, time1));
        //         &x
        //     }
        // };

        match (
            left.as_ref().map(|b| b.bounding_box(time0, time1)),
            right.as_ref().map(|b| b.bounding_box(time0, time1)),
        ) {
            (Some(Some(left_box)), Some(Some(right_box))) => Self {
                left,
                right,
                bounding_box: left_box.surrounding_box(&right_box),
            },
            _ => exit(0),
        }
    }

    fn left(&self) -> &Option<Arc<dyn Hittable>> {
        &self.left
    }

    fn right(&self) -> &Option<Arc<dyn Hittable>> {
        &self.right
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hit::HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let rec_left = self.left.as_ref().map(|b| b.hit(ray, t_min, t_max));

        let right_t_max = match &rec_left {
            Some(Some(rec)) => rec.t,
            _ => t_max,
        };

        let rec_right =
            self.right.as_ref().map(|b| b.hit(ray, t_min, right_t_max));

        match (rec_left, rec_right) {
            (_, Some(Some(rec))) => Some(rec),
            (Some(Some(rec)), _) => Some(rec),
            _ => None,
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.bounding_box)
    }
}

impl BvhNode {
    fn box_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
        axis: usize,
    ) -> Ordering {
        let a_bounding_box = a.bounding_box(0.0, 0.0);
        let b_bounding_box = b.bounding_box(0.0, 0.0);

        match (a_bounding_box, b_bounding_box) {
            (Some(box_a), Some(box_b)) => {
                box_a.min()[axis].partial_cmp(&box_b.min()[axis]).unwrap()
            }
            _ => Ordering::Equal,
        }
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}
