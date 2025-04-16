use std::fmt;
use trigo::angle::Angle;

use crate::consts::{A, A_BAR, ALPHA, TSNOPN, R, N, BETA, DELTA, M_MAX, M_0};
use crate::error::{CoordError, Result};


#[derive(PartialEq, Clone, Copy, Debug)]
pub struct LatLon {
    lat: Angle<f64>,
    lon: Angle<f64>,
}

impl LatLon {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        let lat = Angle::from_deg(latitude);
        let lon = Angle::from_deg(longitude);
        Self::from_angle(lat, lon)
    }
    pub fn from_angle(lat: Angle<f64>, lon: Angle<f64>) -> Self {
        let mut ret = Self { lat, lon };
        ret.bind();
        ret
    }
    pub fn lat(&self) -> Angle<f64> { self.lat }
    pub fn lon(&self) -> Angle<f64> { self.lon }
    pub fn bind(&mut self) {
        self.lat.bind();
        self.lon.bind();
        let lat_deg = self.lat.deg();
        if lat_deg > 90.0 { self.lat = Angle::from_deg(180.0 - lat_deg); }
        else if lat_deg < -90.0 { self.lat = -Angle::from_deg(180.0 + lat_deg); }
    }
    pub fn to_xy(self, reference: Self) -> Result<XY> {
        let (lat, lon) = (self.lat, self.lon);
        let (lat_0, lon_0) = (reference.lat, reference.lon);

        let t = (Angle::atanh(lat.sin()) - TSNOPN*Angle::atanh(TSNOPN*lat.sin())).sinh();
        let t_bar = (1.0 + t*t).sqrt();
        let lon_c = (lon - lon_0).cos();
        let lon_s = (lon - lon_0).sin();
        let xi = Angle::atan(t / lon_c);
        let eta = Angle::atanh(lon_s / t_bar);
        let sigma = 1.0 + (1..6).map(|j| (2*j)as f64*ALPHA[j]*((2*j)as f64*xi).cos()*((2*j)as f64*eta).cosh()).sum::<f64>();
        let tau = (1..6).map(|j| (2*j)as f64*ALPHA[j]*((2*j)as f64*xi).sin()*((2*j)as f64*eta).sinh()).sum::<f64>();

        let s_bar = A_BAR * (lat_0.rad() + (1..6).map(|j| A[j]*((2*j)as f64*lat_0).sin()).sum::<f64>()/A[0]);
        let x = A_BAR * (xi.rad() + (1..6).map(|j| ALPHA[j]*((2*j)as f64*xi).sin()*((2*j)as f64*eta).cosh()).sum::<f64>()) - s_bar;
        let y = A_BAR * (eta.rad() + (1..6).map(|j| ALPHA[j]*((2*j)as f64*xi).cos()*((2*j)as f64*eta).sinh()).sum::<f64>());
        let gamma = Angle::atan((tau*t_bar*lon_c + sigma*t*lon_s) / (sigma*t_bar*lon_c - tau*t*lon_s));
        let on2tanlat = (1.0 - N[1])/(1.0 + N[1])*lat.tan();
        let m = A_BAR/R * ((sigma*sigma+tau*tau)/(t*t+lon_c*lon_c)*(1.0 + on2tanlat*on2tanlat)).sqrt();

        if m > M_MAX { Err(CoordError::Distorted(m, M_MAX)) }
        else {
            let north = -gamma;
            let scale = m;
            let source = self;
            Ok(XY { x, y, north, scale, source, reference })
        }
    }
    pub fn direction_sphere(&self, to: Self) -> (f64, Angle<f64>) {
        let (lat_0, lon_0) = (self.lat, self.lon);
        let (lat, lon) = (to.lat, to.lon);
        let d_lon = lon - lon_0;
        let distance = R * (lat_0.sin()*lat.sin() + lat_0.cos()*lat.cos()*d_lon.cos()).acos();
        let angle = Angle::from_deg(90.0) - Angle::atan2(
            lat_0.cos()*lat.tan() - lat_0.sin()*d_lon.cos(),
            d_lon.sin()
        );
        (distance, angle)
    }
    pub fn direction_plane(self, to: Self) -> Result<(f64, Angle<f64>)> {
        let xy_0 = XY { x: 0.0, y: 0.0, north: Angle::zero(), scale: M_0, source: self, reference: self };
        let xy = XY::from_latlon(to, self)?;
        xy_0.direction(xy)
    }
}

impl fmt::Display for LatLon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.lat.deg(), self.lon.deg())
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct XY {
    x: f64,
    y: f64,
    north: Angle<f64>,
    scale: f64,
    source: LatLon,
    reference: LatLon,
}

impl XY {
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn north(&self) -> Angle<f64> { self.north }
    pub fn scale(&self) -> f64 { self.scale }
    pub fn source(&self) -> LatLon { self.source }
    pub fn reference(&self) -> LatLon { self.reference }

    pub fn zero(reference: LatLon) -> Self {
        let source = reference;
        Self { x: 0.0, y: 0.0, north: Angle::zero(), scale: M_0, source, reference }
    }
    pub fn from_latlon(source: LatLon, reference: LatLon) -> Result<Self> {
        source.to_xy(reference)
    }
    pub fn from_xy(x: f64, y: f64, reference: LatLon) -> Result<Self> {
        let (lat_0, lon_0) = (reference.lat, reference.lon);

        let s_bar = A_BAR * (lat_0.rad() + (1..6).map(|j| A[j]*((2*j)as f64*lat_0).sin()).sum::<f64>()/A[0]);
        let xi = (x + s_bar) / A_BAR;
        let eta = y / A_BAR;
        let xi_ = xi - (1..6).map(|j| BETA[j]*((2*j)as f64*xi).sin()*((2*j)as f64*eta).cosh()).sum::<f64>();
        let eta_ = eta - (1..6).map(|j| BETA[j]*((2*j)as f64*xi).cos()*((2*j)as f64*eta).sinh()).sum::<f64>();
        let sigma_ = 1.0 - (1..6).map(|j| (2*j)as f64*BETA[j]*((2*j)as f64*xi).cos()*((2*j)as f64*eta).cosh()).sum::<f64>();
        let tau_ = (1..6).map(|j| (2*j)as f64*BETA[j]*((2*j)as f64*xi).sin()*((2*j)as f64*eta).sinh()).sum::<f64>();
        let chi = Angle::asin(xi_.sin() / eta_.cosh());

        let lat = chi + Angle::from_rad((1..7).map(|j| DELTA[j]*((2*j)as f64*chi).sin()).sum::<f64>());
        let lon = lon_0 + Angle::atan(eta_.sinh() / xi_.cos());
        let gamma = Angle::atan((tau_ + sigma_*xi_.tan()*eta_.tanh()) / (sigma_ - tau_*xi_.tan()*eta_.tanh()));
        let cosxi_ = xi_.cos();
        let sinheta_ = eta_.sinh();
        let on2tanlat = (1.0 - N[1])/(1.0 + N[1])*lat.tan();
        let m = A_BAR/R * ((cosxi_*cosxi_ + sinheta_*sinheta_)/(sigma_*sigma_ + tau_*tau_)*(1.0 + on2tanlat*on2tanlat)).sqrt();

        if m > M_MAX {
            Err(CoordError::Distorted(m, M_MAX))
        } else {
            let north = -gamma;
            let scale = m;
            let source = LatLon::from_angle(lat, lon);
            Ok(Self { x, y, north, scale, source, reference })
        }
    }
    pub fn direction(&self, to: Self) -> Result<(f64, Angle<f64>)> {
        if self.reference != to.reference { return Err(CoordError::DiffCoord(self.reference, to.reference)); }
        let (dx, dy) = (to.x - self.x, to.y - self.y);
        Ok(((dx*dx + dy*dy).sqrt(), Angle::from_deg(90.0) - Angle::atan2(dx, dy)))
    }
}