use coordinate::functions::circle_center;
use coordinate::point::{LatLon, XY};
use trigo::angle::Angle;
use trigo::dms::DMS;
use coordinate::consts::*;

fn main() {
    println!("{:?}", DELTA);
    /*let reference = LatLon::from_angle(
        Angle::from_dms(DMS::new(true, 36, 0, 0.0)),
        Angle::from_dms(DMS::new(true, 139, 50, 0.0)),
    );
    let point = LatLon::new(36.103774791666666, 140.08785504166664);
    let xy = point.to_xy(reference).unwrap();
    println!("{}, {}, {}, {}", xy.x(), xy.y(), xy.north().deg(), xy.scale());

    let xy_ = XY::from_xy(xy.x(), xy.y(), reference).unwrap();

    println!("{}", DMS::from_decimal(xy_.source().lat().deg()));
    println!("{}", DMS::from_decimal(xy_.source().lon().deg()));
    println!("{}", DMS::from_decimal(xy_.north().deg()));
    println!("{}", xy_.scale());

    let a = LatLon::new(34.4706978,136.6937732);
    let b = LatLon::new(33.8405858,135.5446009);

    let (d_s, a_s) = a.direction_sphere(b);
    let (d_p, a_p) = a.direction_plane(b).unwrap();
    println!("{} {}", d_s, a_s.to_dms());
    println!("{} {}", d_p, a_p.to_dms());

    let c = LatLon::new(34.4600537,134.8499204);
    let (center, distance) = circle_center(a, b, c).unwrap();
    println!("{} {}", center.lat().deg(), center.lon().deg());
    println!("{}", distance);
    println!("{}", center.direction_plane(a).unwrap().0);
    println!("{}", center.direction_plane(b).unwrap().0);
    println!("{}", center.direction_plane(c).unwrap().0);*/
}