use std::marker::PhantomData;

use half::f16;
use ngt_sys as sys;
use num_enum::TryFromPrimitive;

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(i32)]
pub enum QbgObject {
    Uint8 = 0,
    Float = 1,
    Float16 = 2,
}

mod private {
    pub trait Sealed {}
}

pub trait QbgObjectType: private::Sealed {
    fn as_obj() -> QbgObject;
}

impl private::Sealed for f32 {}
impl QbgObjectType for f32 {
    fn as_obj() -> QbgObject {
        QbgObject::Float
    }
}

impl private::Sealed for u8 {}
impl QbgObjectType for u8 {
    fn as_obj() -> QbgObject {
        QbgObject::Uint8
    }
}

impl private::Sealed for f16 {}
impl QbgObjectType for f16 {
    fn as_obj() -> QbgObject {
        QbgObject::Float16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(i32)]
pub enum QbgDistance {
    L2 = 1,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QbgConstructParams<T> {
    extended_dimension: usize,
    dimension: usize,
    number_of_subvectors: usize,
    number_of_blobs: usize,
    internal_data_type: QbgObject,
    data_type: QbgObject,
    distance_type: QbgDistance,
    _marker: PhantomData<T>,
}

impl<T> QbgConstructParams<T>
where
    T: QbgObjectType,
{
    pub fn dimension(dimension: usize) -> Self {
        let extended_dimension = next_multiple_of_16(dimension);
        let number_of_subvectors = 1;
        let number_of_blobs = 0;
        let internal_data_type = T::as_obj();
        let data_type = T::as_obj();
        let distance_type = QbgDistance::L2;

        Self {
            extended_dimension,
            dimension,
            number_of_subvectors,
            number_of_blobs,
            internal_data_type,
            data_type,
            distance_type,
            _marker: PhantomData,
        }
    }

    pub fn extended_dimension(mut self, extended_dimension: usize) -> Result<Self, Error> {
        if extended_dimension % 16 == 0 && extended_dimension >= self.dimension {
            self.extended_dimension = extended_dimension;
            Ok(self)
        } else {
            Err(Error(format!(
                "Invalid extended_dimension: {}, must be a multiple of 16 greater or equal to dimension",
                extended_dimension
            )))
        }
    }

    pub fn number_of_subvectors(mut self, number_of_subvectors: usize) -> Self {
        self.number_of_subvectors = number_of_subvectors;
        self
    }

    pub fn number_of_blobs(mut self, number_of_blobs: usize) -> Self {
        self.number_of_blobs = number_of_blobs;
        self
    }

    pub fn internal_data_type(mut self, internal_data_type: QbgObject) -> Self {
        self.internal_data_type = internal_data_type;
        self
    }

    pub fn distance_type(mut self, distance_type: QbgDistance) -> Self {
        self.distance_type = distance_type;
        self
    }

    pub(crate) unsafe fn into_raw(self) -> sys::QBGConstructionParameters {
        sys::QBGConstructionParameters {
            extended_dimension: self.extended_dimension,
            dimension: self.dimension,
            number_of_subvectors: self.number_of_subvectors,
            number_of_blobs: self.number_of_blobs,
            internal_data_type: self.internal_data_type as i32,
            data_type: self.data_type as i32,
            distance_type: self.distance_type as i32,
        }
    }
}

fn next_multiple_of_16(x: usize) -> usize {
    ((x + 15) / 16) * 16
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(i32)]
pub enum QbgClusteringInitMode {
    Head = 0,
    Random = 1,
    KmeansPlusPlus = 2,
    RandomFixedSeed = 3,
    KmeansPlusPlusFixedSeed = 4,
    Best = 5,
}

#[derive(Debug, Clone)]
pub struct QbgBuildParams {
    // hierarchical kmeans
    hierarchical_clustering_init_mode: QbgClusteringInitMode,
    number_of_first_objects: usize,
    number_of_first_clusters: usize,
    number_of_second_objects: usize,
    number_of_second_clusters: usize,
    number_of_third_clusters: usize,
    // optimization
    number_of_objects: usize,
    number_of_subvectors: usize,
    optimization_clustering_init_mode: QbgClusteringInitMode,
    rotation_iteration: usize,
    subvector_iteration: usize,
    number_of_matrices: usize,
    rotation: bool,
    repositioning: bool,
}

impl Default for QbgBuildParams {
    fn default() -> Self {
        Self {
            hierarchical_clustering_init_mode: QbgClusteringInitMode::KmeansPlusPlus,
            number_of_first_objects: 0,
            number_of_first_clusters: 0,
            number_of_second_objects: 0,
            number_of_second_clusters: 0,
            number_of_third_clusters: 0,
            number_of_objects: 1000,
            number_of_subvectors: 1,
            optimization_clustering_init_mode: QbgClusteringInitMode::KmeansPlusPlus,
            rotation_iteration: 2000,
            subvector_iteration: 400,
            number_of_matrices: 3,
            rotation: true,
            repositioning: false,
        }
    }
}

impl QbgBuildParams {
    pub fn hierarchical_clustering_init_mode(
        mut self,
        clustering_init_mode: QbgClusteringInitMode,
    ) -> Self {
        self.hierarchical_clustering_init_mode = clustering_init_mode;
        self
    }

    pub fn number_of_first_objects(mut self, number_of_first_objects: usize) -> Self {
        self.number_of_first_objects = number_of_first_objects;
        self
    }

    pub fn number_of_first_clusters(mut self, number_of_first_clusters: usize) -> Self {
        self.number_of_first_clusters = number_of_first_clusters;
        self
    }

    pub fn number_of_second_objects(mut self, number_of_second_objects: usize) -> Self {
        self.number_of_second_objects = number_of_second_objects;
        self
    }

    pub fn number_of_second_clusters(mut self, number_of_second_clusters: usize) -> Self {
        self.number_of_second_clusters = number_of_second_clusters;
        self
    }

    pub fn number_of_third_clusters(mut self, number_of_third_clusters: usize) -> Self {
        self.number_of_third_clusters = number_of_third_clusters;
        self
    }

    pub fn number_of_objects(mut self, number_of_objects: usize) -> Self {
        self.number_of_objects = number_of_objects;
        self
    }
    pub fn number_of_subvectors(mut self, number_of_subvectors: usize) -> Self {
        self.number_of_subvectors = number_of_subvectors;
        self
    }
    pub fn optimization_clustering_init_mode(
        mut self,
        clustering_init_mode: QbgClusteringInitMode,
    ) -> Self {
        self.optimization_clustering_init_mode = clustering_init_mode;
        self
    }

    pub fn rotation_iteration(mut self, rotation_iteration: usize) -> Self {
        self.rotation_iteration = rotation_iteration;
        self
    }

    pub fn subvector_iteration(mut self, subvector_iteration: usize) -> Self {
        self.subvector_iteration = subvector_iteration;
        self
    }

    pub fn number_of_matrices(mut self, number_of_matrices: usize) -> Self {
        self.number_of_matrices = number_of_matrices;
        self
    }

    pub fn rotation(mut self, rotation: bool) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn repositioning(mut self, repositioning: bool) -> Self {
        self.repositioning = repositioning;
        self
    }

    pub(crate) unsafe fn into_raw(self) -> sys::QBGBuildParameters {
        sys::QBGBuildParameters {
            hierarchical_clustering_init_mode: self.hierarchical_clustering_init_mode as i32,
            number_of_first_objects: self.number_of_first_objects,
            number_of_first_clusters: self.number_of_first_clusters,
            number_of_second_objects: self.number_of_second_objects,
            number_of_second_clusters: self.number_of_second_clusters,
            number_of_third_clusters: self.number_of_third_clusters,
            number_of_objects: self.number_of_objects,
            number_of_subvectors: self.number_of_subvectors,
            optimization_clustering_init_mode: self.optimization_clustering_init_mode as i32,
            rotation_iteration: self.rotation_iteration,
            subvector_iteration: self.subvector_iteration,
            number_of_matrices: self.number_of_matrices,
            rotation: self.rotation,
            repositioning: self.repositioning,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qbg_params() {
        let params = QbgConstructParams::<f32>::dimension(3);
        assert_eq!(params.extended_dimension, 16);

        let params = QbgConstructParams::<u8>::dimension(16);
        assert_eq!(params.extended_dimension, 16);

        let params = QbgConstructParams::<f32>::dimension(513);
        assert_eq!(params.extended_dimension, 528);
    }
}
