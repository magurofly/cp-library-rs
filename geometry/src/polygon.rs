use super::*;
use num_traits::*;

/// 格子多角形の面積を計算する
/// - `pos`: 頂点の座標（時計回りまたは反時計回りのどちらか）
pub fn polygon_area<T: Num + PartialOrd + Clone>(pos: &[impl Pos2D<T>]) -> T {
    area(pos).0 / (T::one() + T::one())
}

/// 格子多角形の座標の並びが時計回りか反時計回りか判定する
/// - `pos`: 頂点の座標（時計回りまたは反時計回りのどちらか）
pub fn is_clockwise<T: Num + PartialOrd + Clone>(pos: &[impl Pos2D<T>]) -> bool {
    area(pos).1
}

fn area<T: Num + PartialOrd + Clone>(pos: &[impl Pos2D<T>]) -> (T, bool) {
    let mut area = T::zero();
    let mut sign = false;
    for i in 0..pos.len() {
        let p = &pos[i];
        let q = &pos[(i + 1) % pos.len()];
        let a = p.x() * q.y();
        let b = p.y() * q.x();
        if sign {
            area = area + b;
            if area < a {
                area = a - area;
                sign = false;
            }
        } else {
            area = area + a;
            if area < b {
                area = b - area;
                sign = true;
            }
        }
    }
    (area, sign)
}
