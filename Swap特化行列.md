# $O(1)$ でできること
- 行・列の swap, reverse
- 転置
- 左右回転

# コード
```rs
#[derive(Clone)]
pub struct Matrix<T> {
    n: usize, ip: Vec<usize>, ii: isize, ij: isize, ic: isize, ki: usize,
    m: usize, jp: Vec<usize>, ji: isize, jj: isize, jc: isize, kj: usize,
    a: Vec<T>,
}
impl<T> Matrix<T> {
    pub fn new(n: usize, m: usize, mut f: impl FnMut(usize, usize) -> T) -> Self {
        Self {
            n, ip: (0 .. n).collect(), ii: 1, ij: 0, ic: 0, ki: m,
            m, jp: (0 .. m).collect(), ji: 0, jj: 1, jc: 0, kj: 1,
            a: (0 .. n).flat_map(|i| (0 .. m).map(move |j| (i, j) ) ).map(|(i, j)| f(i, j) ).collect(),
        }
    }

    pub fn from_slice(n: usize, m: usize, slice: &[T]) -> Self where T: Clone {
        assert!(slice.len() >= n * m);
        Self::new(n, m, |i, j| slice[i * m + j].clone() )
    }

    /// number of rows
    pub fn n(&self) -> usize {
        self.n
    }

    /// number of columns
    pub fn m(&self) -> usize {
        self.m
    }

    pub fn reverse_rows(&mut self) {
        self.ii *= -1;
        self.ij *= -1;
        self.ic = -self.ic - 1;
    }

    pub fn reverse_cols(&mut self) {
        self.ji *= -1;
        self.jj *= -1;
        self.jc = -self.jc - 1;
    }

    pub fn swap_rows(&mut self, i1: usize, i2: usize) {
        assert!(i1 < self.n && i2 < self.m);
        self.ip.swap(i1, i2);
    }

    pub fn swap_cols(&mut self, j1: usize, j2: usize) {
        assert!(j1 < self.n && j2 < self.m);
        self.jp.swap(j1, j2);
    }

    pub fn transpose(&mut self) {
        std::mem::swap(&mut self.n, &mut self.m);
        std::mem::swap(&mut self.ip, &mut self.jp);
        std::mem::swap(&mut self.ii, &mut self.jj);
        std::mem::swap(&mut self.ij, &mut self.ji);
        std::mem::swap(&mut self.ic, &mut self.jc);
        std::mem::swap(&mut self.ki, &mut self.kj);
    }

    pub fn rotate_r(&mut self) {
        self.reverse_rows();
        self.transpose();
    }

    pub fn rotate_l(&mut self) {
        self.reverse_cols();
        self.transpose();
    }

    pub fn affine(&mut self, ii: isize, ij: isize, ic: isize, ji: isize, jj: isize, jc: isize) {
        let nm = (self.n * self.m) as isize;
        assert!((ii * jj - ij * ji) % nm != 0);
        self.ii = (ii * self.ii + ij * self.ji) % nm;
        self.ij = (ii * self.ij + ij * self.jj) % nm;
        self.ic = (ii * self.ic + ij * self.jc + ic) % nm;
        self.ji = (ji * self.ii + jj * self.ji) % nm;
        self.jj = (ji * self.ij + jj * self.jj) % nm;
        self.jc = (ji * self.ic + jj * self.jc + jc) % nm;
    }

    pub fn map<U>(&self, mut f: impl FnMut(usize, usize, &T) -> U) -> Matrix<U> {
        Matrix::new(self.n, self.m, |i, j| f(i, j, &self[(i, j)]) )
    }

    pub fn iter(&self) -> <&Self as std::iter::IntoIterator>::IntoIter {
        self.into_iter()
    }

    pub fn neighbor4(&self, i: usize, j: usize, mut f: impl FnMut(usize, usize, &T)) {
        assert!(i < self.n && j < self.m);
        if i > 0 {
            f(i - 1, j, &self[(i - 1, j)]);
        }
        if j > 0 {
            f(i, j - 1, &self[(i, j - 1)]);
        }
        if i + 1 < self.n {
            f(i + 1, j, &self[(i + 1, j)]);
        }
        if j + 1 < self.m {
            f(i, j + 1, &self[(i, j + 1)]);
        }
    }

    fn k(&self, mut i: usize, mut j: usize) -> usize {
        assert!(i < self.n && j < self.m);
        let _i = self.ip[i] as isize;
        let _j = self.jp[j] as isize;
        i = (self.ii * _i + self.ij * _j + self.ic).rem_euclid(self.n as isize) as usize;
        j = (self.ji * _i + self.jj * _j + self.jc).rem_euclid(self.m as isize) as usize;
        self.ki * i + self.kj * j
    }
}
impl<T> Matrix<T> {
    pub fn pow(&self, mut exp: usize) -> Self where T: Clone + std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T> {
        assert!(self.n == self.m);
        assert!(exp > 0);
        exp -= 1;
        let mut r = self.clone();
        let mut a = self.clone();
        while exp > 0 {
            if exp & 1 != 0 {
                r = &r * &a;
            }
            a = &a * &a;
            exp >>= 1;
        }
        r
    }
}
impl<T> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, (i, j): (usize, usize)) -> &T {
        assert!(i < self.n && j < self.m);
        &self.a[self.k(i, j)]
    }
}
impl<T> std::ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        assert!(i < self.n && j < self.m);
        let k = self.k(i, j);
        &mut self.a[k]
    }
}
impl<T> std::ops::Add<Self> for &Matrix<T> where T: Clone + std::ops::Add<T, Output = T> {
    type Output = Matrix<T>;
    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.n == rhs.n && self.m == rhs.m);
        Matrix::new(self.n, self.m, |i, j| self[(i, j)].clone() + rhs[(i, j)].clone() )
    }
}
impl<T> std::ops::AddAssign<&Self> for Matrix<T> where T: Clone + std::ops::AddAssign<T> {
    fn add_assign(&mut self, rhs: &Self) {
        assert!(self.n == rhs.n && self.m == rhs.m);
        for i in 0 .. self.n {
            for j in 0 .. self.m {
                self[(i, j)] += rhs[(i, j)].clone();
            }
        }
    }
}
impl<T> std::ops::Sub<Self> for &Matrix<T> where T: Clone + std::ops::Sub<T, Output = T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.n == rhs.n && self.m == rhs.m);
        Matrix::new(self.n, self.m, |i, j| self[(i, j)].clone() - rhs[(i, j)].clone() )
    }
}
impl<T> std::ops::SubAssign<&Self> for Matrix<T> where T: Clone + std::ops::SubAssign<T> {
    fn sub_assign(&mut self, rhs: &Self) {
        assert!(self.n == rhs.n && self.m == rhs.m);
        for i in 0 .. self.n {
            for j in 0 .. self.m {
                self[(i, j)] -= rhs[(i, j)].clone();
            }
        }
    }
}
impl<T> std::ops::MulAssign<&T> for Matrix<T> where T: Clone + std::ops::MulAssign<T> {
    fn mul_assign(&mut self, rhs: &T) {
        for i in 0 .. self.n {
            for j in 0 .. self.m {
                self[(i, j)] *= rhs.clone();
            }
        }
    }
}
impl<T> std::ops::Mul<&Matrix<T>> for &Matrix<T> where T: Clone + std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        assert!(self.m == rhs.n);
        assert!(self.m > 0);
        Matrix::new(self.n, rhs.m, |i, k| (0 .. self.m).map(|j| self[(i, j)].clone() * rhs[(j, k)].clone() ).reduce(|x, y| x + y ).expect("self.m > 0") )
    }
}
impl<T: std::fmt::Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::*;
        if self.n == 0 {
            return f.write_str("[]");
        }
        let a = (0 .. self.n).map(|i| (0 .. self.m).map(|j| format!("{:?}", self[(i, j)]) ).collect() ).collect::<Vec<Vec<_>>>();
        let col_ws = (0 .. self.m).map(|j| (0 .. self.n).map(|i| a[i][j].len() ).max().unwrap() ).collect::<Vec<_>>();
        for i in 0 .. self.n {
            if i == 0 {
                f.write_char('[')?;
            } else {
                f.write_char(' ')?;
            }
            for j in 0 .. self.m {
                f.write_str(&a[i][j])?;
                f.write_str(&" ".repeat(col_ws[j] - a[i][j].len()))?;
                if j == self.m - 1 {
                    if i == self.n - 1 {
                        f.write_str("]")?;
                    } else {
                        f.write_str(",\n")?;
                    }
                } else {
                    f.write_char(' ')?;
                }
            }
        }
        Ok(())
    }
}
impl<'a, T> std::iter::IntoIterator for &'a Matrix<T> {
    type Item = (usize, usize, &'a T);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        (0 .. self.n).flat_map(|i| (0 .. self.m).map(move |j| (i, j, &self[(i, j)]) ) ).collect::<Vec<_>>().into_iter()
    }
}
```
