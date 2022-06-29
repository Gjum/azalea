use azalea_core::Axis;

use crate::{BitSetDiscreteVoxelShape, DiscreteVoxelShape, AABB, EPSILON};
use std::ops::Add;

pub struct Shapes {}

pub fn block_shape() -> Box<dyn VoxelShape> {
    let mut shape = BitSetDiscreteVoxelShape::new(1, 1, 1);
    shape.fill(0, 0, 0);
    Box::new(CubeVoxelShape::new(Box::new(shape)))
}
pub fn empty_shape() -> Box<dyn VoxelShape> {
    Box::new(ArrayVoxelShape::new(
        Box::new(BitSetDiscreteVoxelShape::new(0, 0, 0)),
        vec![0.],
        vec![0.],
        vec![0.],
    ))
}

impl Shapes {
    pub fn collide_x(
        entity_box: &AABB,
        collision_boxes: &Vec<Box<dyn VoxelShape>>,
        movement: f64,
    ) -> f64 {
        let mut shape: Box<dyn VoxelShape>;
        for shape in collision_boxes {
            if movement.abs() < EPSILON {
                return 0.;
            }
            movement = shape.collide_x(entity_box, movement);
        }
        movement
    }
}

pub trait VoxelShape {
    fn shape(&self) -> Box<dyn DiscreteVoxelShape>;

    fn get_x_coords(&self) -> Vec<f64>;
    fn get_y_coords(&self) -> Vec<f64>;
    fn get_z_coords(&self) -> Vec<f64>;

    // TODO: optimization: should this be changed to return ArrayVoxelShape?
    // i might change the implementation of empty_shape in the future so not 100% sure
    fn move_relative(&self, x: f64, y: f64, z: f64) -> Box<dyn VoxelShape> {
        if self.shape().is_empty() {
            return empty_shape();
        }
        Box::new(ArrayVoxelShape::new(
            self.shape(),
            self.get_x_coords().iter().map(|c| c + x).collect(),
            self.get_y_coords().iter().map(|c| c + y).collect(),
            self.get_z_coords().iter().map(|c| c + z).collect(),
        ))
    }

    fn collide(axis: &Axis, entity_box: &AABB, movement: f64) {
        self.collide_x()
    }
}

pub struct ArrayVoxelShape {
    shape: Box<dyn DiscreteVoxelShape>,
    faces: Option<Vec<Box<dyn VoxelShape>>>,

    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
    pub zs: Vec<f64>,
}

pub struct CubeVoxelShape {
    shape: Box<dyn DiscreteVoxelShape>,
    faces: Option<Vec<Box<dyn VoxelShape>>>,
}

impl ArrayVoxelShape {
    pub fn new(
        shape: Box<dyn DiscreteVoxelShape>,
        xs: Vec<f64>,
        ys: Vec<f64>,
        zs: Vec<f64>,
    ) -> Self {
        let x_size = shape.x_size() + 1;
        let y_size = shape.y_size() + 1;
        let z_size = shape.z_size() + 1;

        // Lengths of point arrays must be consistent with the size of the VoxelShape.
        assert_eq!(x_size, xs.len() as u32);
        assert_eq!(y_size, ys.len() as u32);
        assert_eq!(z_size, zs.len() as u32);

        Self {
            faces: None,
            shape,
            xs,
            ys,
            zs,
        }
    }
}

impl CubeVoxelShape {
    pub fn new(shape: Box<dyn DiscreteVoxelShape>) -> Self {
        Self { shape, faces: None }
    }
}

impl VoxelShape for ArrayVoxelShape {
    fn shape(&self) -> Box<dyn DiscreteVoxelShape> {
        self.shape.clone()
    }

    fn get_x_coords(&self) -> Vec<f64> {
        self.xs.clone()
    }

    fn get_y_coords(&self) -> Vec<f64> {
        self.ys.clone()
    }

    fn get_z_coords(&self) -> Vec<f64> {
        self.zs.clone()
    }
}

impl VoxelShape for CubeVoxelShape {
    fn shape(&self) -> Box<dyn DiscreteVoxelShape> {
        self.shape.clone()
    }

    fn get_x_coords(&self) -> Vec<f64> {
        let size = self.shape.x_size();
        let mut parts = Vec::with_capacity(size as usize);
        for i in 0..size {
            parts.push(i as f64 / size as f64);
        }
        parts
    }

    fn get_y_coords(&self) -> Vec<f64> {
        let size = self.shape.y_size();
        let mut parts = Vec::with_capacity(size as usize);
        for i in 0..size {
            parts.push(i as f64 / size as f64);
        }
        parts
    }

    fn get_z_coords(&self) -> Vec<f64> {
        let size = self.shape.z_size();
        let mut parts = Vec::with_capacity(size as usize);
        for i in 0..size {
            parts.push(i as f64 / size as f64);
        }
        parts
    }
}
