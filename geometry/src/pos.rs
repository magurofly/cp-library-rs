use num_traits::Num;

pub trait Pos2D<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
}

impl<T: Num + Clone> Pos2D<T> for (T, T) {
    fn x(&self) -> T {
        self.0.clone()
    }

    fn y(&self) -> T {
        self.1.clone()
    }
}
