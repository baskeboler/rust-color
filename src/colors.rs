use std::fmt::{Display, Error};
use std::result::Result;

pub trait Color {
    type Type;
    fn complement(&self) -> Self::Type;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbaColorType {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HslaColorType {
    h: f64,
    s: f64,
    l: f64,
    a: f64,
}

impl RgbaColorType {
    pub fn new(r: f64, g: f64, b: f64) -> RgbaColorType {
        RgbaColorType {
            r: r,
            g: g,
            b: b,
            a: 1.0,
        }
    }
}

impl HslaColorType {
    pub fn new(h: f64, s: f64, l: f64) -> HslaColorType {
        let mut a = HslaColorType {
            h: h,
            s: s,
            l: l,
            a: 1.0,
        };
        a.validate();
        a
    }

    fn validate(&mut self) {
        self.h = self.h % 360.0;
        if self.h < 0.0 {
            self.h += 360.0
        };

        if self.s < 0.0 {
            self.s = 0.0
        };
        if self.s > 100.0 {
            self.s = 100.0
        };
        if self.l < 0.0 {
            self.l = 0.0
        };
        if self.l > 100.0 {
            self.l = 100.0
        };
    }
}
fn hue2rgb(p: f64, q: f64, t: f64) -> f64 {
    let mut t = t;
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

impl From<[f64; 3]> for RgbaColorType {
    fn from(c: [f64; 3]) -> RgbaColorType {
        RgbaColorType::new(c[0], c[1], c[2])
    }
}
impl From<[u8; 3]> for RgbaColorType {
    fn from(c: [u8; 3]) -> RgbaColorType {
        RgbaColorType::new(
            (c[0] as f64 / 255.0),
            (c[1] as f64 / 255.0),
            (c[2] as f64 / 255.0),
        )
    }
}

impl Into<[u8; 3]> for RgbaColorType {
    fn into(self) -> [u8; 3] {
        [
            (self.r * 255.0).round() as u8,
            (self.g * 255.0).round() as u8,
            (self.b * 255.0).round() as u8,
        ]
    }
}
impl From<HslaColorType> for RgbaColorType {
    fn from(c: HslaColorType) -> RgbaColorType {
        let h: f64 = c.h / 360.0;
        let s: f64 = c.s / 100.0;
        let l: f64 = c.l / 100.0;

        let mut r: f64 = l;
        let mut g: f64 = l;
        let mut b: f64 = l;

        if s != 0.0 {
            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;
            r = hue2rgb(p, q, h + 1.0 / 3.0);
            g = hue2rgb(p, q, h);
            b = hue2rgb(p, q, h - 1.0 / 3.0);
        }
        RgbaColorType::new(r, g, b)
    }
}
impl From<RgbaColorType> for HslaColorType {
    fn from(c: RgbaColorType) -> HslaColorType {
        let mut h: f64;
        let mut s: f64;
        let mut l: f64;
        let mut r: f64 = c.r;
        let mut g: f64 = c.g;
        let mut b: f64 = c.b;

        let max: f64 = c.r.max(c.g).max(c.b);
        let min: f64 = c.r.min(c.g).min(c.b);
        h = (max + min) / 2.0;
        s = h;
        l = h;

        if max == min {
            // achromatic

            h = 0.0;
            s = 0.0;
        } else {
            let d = max - min;
            s = if l > 0.5 {
                d / (2.0 - max - min)
            } else {
                d / (max + min)
            };

            if max == r {
                h = (g - b) / d + (if g < b { 6.0 } else { 0.0 })
            } else if max == g {
                h = (b - r) / d + 2.0
            } else if max == b {
                h = (r - g) / d + 4.0;
            }

            h = h / 6.0;
        }
        h = 360.0 * h;
        s = 100.0 * s;
        l = 100.0 * l;

        HslaColorType::new(h, s, l)
    }
}
impl Display for RgbaColorType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_str(format!("RGB({:?}, {:?}, {:?})", self.r, self.g, self.b).as_str())
    }
}
impl Color for RgbaColorType {
    type Type = RgbaColorType;

    fn complement(&self) -> Self::Type {
        RgbaColorType::new(1.0 - self.r, 1.0 - self.g, 1.0 - self.b)
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let c = RgbaColorType::new(0.1, 0.2, 0.3);
        assert_eq!(
            c,
            RgbaColorType {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0
            }
        );
    }

    #[test]
    fn test_complement() {
        let c = RgbaColorType::new(0.1, 0.2, 0.3);
        assert_eq!(
            c.complement(),
            RgbaColorType {
                r: 0.9,
                g: 0.8,
                b: 0.7,
                a: 1.0
            }
        );
    }

    #[test]
    fn test_from_array() {
        let a: RgbaColorType = RgbaColorType::from([1.0, 0.5, 0.0]);
        assert_eq!(
            a,
            RgbaColorType {
                r: 1.0,
                g: 0.5,
                b: 0.0,
                a: 1.0
            }
        );
    }

    #[test]
    fn test_from_bytes() {
        let a = RgbaColorType::from([255, 255, 255]);
        assert_eq!(
            a,
            RgbaColorType {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0
            }
        );
        let b = RgbaColorType::from([128, 128, 128]);
        assert!(
            error(
                b,
                RgbaColorType {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 1.0
                }
            ) < 0.005
        );
    }
    #[test]
    fn test_into() {
        let a = RgbaColorType::new(1.0, 0.5, 0.25);
        let arr: [u8; 3] = a.into();
        assert_eq!(arr, [255, 128, 64]);
    }
    #[test]
    fn test_conv() {
        let a = RgbaColorType::new(1.0, 0.5, 0.25);
        let b: HslaColorType = a.into();
        assert_eq!(a, b.into());
    }
    fn error(c: RgbaColorType, d: RgbaColorType) -> f64 {
        let mut e = c.r - d.r + c.g - d.g + c.b - d.b + c.a - d.a;
        e / 4.0
    }
}
