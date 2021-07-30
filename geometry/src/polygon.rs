use super::*;
use num_traits::*;

/// 格子多角形の面積を計算する
/// - `pos`: 頂点の座標（時計回りまたは反時計回りのどちらか）
pub fn polygon_area<T: Signed + Clone>(pos: &[impl Pos2D<T>]) -> T {
    area(pos).abs() / (T::one() + T::one())
}

/// 格子多角形の座標の並びが時計回りか反時計回りか判定する
/// - `pos`: 頂点の座標（時計回りまたは反時計回りのどちらか）
pub fn is_clockwise<T: Signed + Clone>(pos: &[impl Pos2D<T>]) -> bool {
    area(pos).is_negative()
}

fn area<T: Signed + Clone>(pos: &[impl Pos2D<T>]) -> T {
    let mut area = T::zero();
    for i in 0..pos.len() {
        let p = &pos[i];
        let q = &pos[(i + 1) % pos.len()];
        area = area + p.x() * q.y() - p.y() * q.x();
    }
    area
}
