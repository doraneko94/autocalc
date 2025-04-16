use thiserror::Error;

use crate::point::LatLon;

pub type Result<T> = std::result::Result<T, CoordError>;

#[derive(Debug, Error)]
pub enum CoordError {
    #[error("Highly distorted: Scale factor is {0} > {1}!")]
    Distorted(f64, f64),
    #[error("Different reference coordinates: {0} & {1}!")]
    DiffCoord(LatLon, LatLon)
}