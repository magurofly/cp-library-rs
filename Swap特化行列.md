# $O(1)$ でできること
- 行・列の swap, reverse
- 転置
- 左右回転

# コード
```rs
#[derive(Clone)]
pub struct Matrix<const N: usize, const M: usize, T> {
    ip: Vec<usize>, i_rev: bool, id: usize,
    jp: Vec<usize>, j_rev: bool, jd: usize,
    a: Vec<T>,
}
impl<const N: usize, const M: usize, T> Matrix<N, M, T> {
    pub fn new(mut f: impl FnMut(usize, usize) -> T) -> Self {
        Self {
            ip: (0 .. N).collect::<Vec<_>>(), i_rev: false, id: M,
            jp: (0 .. M).collect::<Vec<_>>(), j_rev: false, jd: 1,
            a: (0 .. N * M).map(|k| f(k / M, k % M) ).collect::<Vec<_>>(),
        }
    }

    pub fn reverse_rows(&mut self) {
        self.i_rev ^= true;
    }

    pub fn reverse_cols(&mut self) {
        self.j_rev ^= true;
    }

    pub fn swap_rows(&mut self, i1: usize, i2: usize) {
        assert!(i1 < N && i2 < N);
        self.ip.swap(i1, i2);
    }

    pub fn swap_cols(&mut self, j1: usize, j2: usize) {
        assert!(j1 < M && j2 < M);
        self.jp.swap(j1, j2);
    }

    pub fn transpose(self) -> Matrix<M, N, T> {
        Matrix {
            ip: self.jp, i_rev: self.j_rev, id: self.jd,
            jp: self.ip, j_rev: self.i_rev, jd: self.id,
            a: self.a,
        }
    }

    pub fn rotate_r(mut self) -> Matrix<M, N, T> {
        self.reverse_rows();
        self.transpose()
    }

    pub fn rotate_l(mut self) -> Matrix<M, N, T> {
        self.reverse_cols();
        self.transpose()
    }

    fn k(&self, mut i: usize, mut j: usize) -> usize {
        assert!(i < N && j < M);
        if self.i_rev {
            i = N - 1 - i;
        }
        if self.j_rev {
            j = M - 1 - j;
        }
        self.id * self.ip[i] + self.jd * self.jp[j]
    }
}
impl<const N: usize, T> Matrix<N, N, T> {
    pub fn pow(&self, mut exp: usize) -> Self where T: Clone + std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T> {
        assert!(exp > 0);
        exp -= 1;
        let mut r = self.clone();
        let mut a = self.clone();
        while exp > 0 {
            if exp & 1 != 0 {
                r = &r * &a;
            }
            a = &a * &a;
        }
        r
    }
}
impl<const N: usize, const M: usize, T> From<&[T]> for Matrix<N, M, T> where T: Clone {
    fn from(slice: &[T]) -> Self {
        assert!(slice.len() >= N * M);
        Self::new(|i, j| slice[i * M + j].clone() )
    }
}
impl<const N: usize, const M: usize, T> std::ops::Index<(usize, usize)> for Matrix<N, M, T> {
    type Output = T;
    fn index(&self, (i, j): (usize, usize)) -> &T {
        assert!(i < N && j < M);
        &self.a[self.k(i, j)]
    }
}
impl<const N: usize, const M: usize, T> std::ops::IndexMut<(usize, usize)> for Matrix<N, M, T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        assert!(i < N && j < M);
        let k = self.k(i, j);
        &mut self.a[k]
    }
}
impl<const N: usize, const M: usize, T> std::ops::Add<Self> for &Matrix<N, M, T> where T: Clone + std::ops::Add<T, Output = T> {
    type Output = Matrix<N, M, T>;
    fn add(self, rhs: Self) -> Self::Output {
        Matrix::new(|i, j| self[(i, j)].clone() + rhs[(i, j)].clone() )
    }
}
impl<const N: usize, const M: usize, T> std::ops::AddAssign<&Self> for Matrix<N, M, T> where T: Clone + std::ops::AddAssign<T> {
    fn add_assign(&mut self, rhs: &Self) {
        for i in 0 .. N {
            for j in 0 .. M {
                self[(i, j)] += rhs[(i, j)].clone();
            }
        }
    }
}
impl<const N: usize, const M: usize, T> std::ops::Sub<Self> for &Matrix<N, M, T> where T: Clone + std::ops::Sub<T, Output = T> {
    type Output = Matrix<N, M, T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix::new(|i, j| self[(i, j)].clone() - rhs[(i, j)].clone() )
    }
}
impl<const N: usize, const M: usize, T> std::ops::SubAssign<&Self> for Matrix<N, M, T> where T: Clone + std::ops::SubAssign<T> {
    fn sub_assign(&mut self, rhs: &Self) {
        for i in 0 .. N {
            for j in 0 .. M {
                self[(i, j)] -= rhs[(i, j)].clone();
            }
        }
    }
}
impl<const N: usize, const M: usize, T> std::ops::MulAssign<T> for Matrix<N, M, T> where T: Clone + std::ops::MulAssign<T> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0 .. N {
            for j in 0 .. M {
                self[(i, j)] *= rhs.clone();
            }
        }
    }
}
impl<const N: usize, const M: usize, const K: usize, T> std::ops::Mul<&Matrix<M, K, T>> for &Matrix<N, M, T> where T: Clone + std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T> {
    type Output = Matrix<N, K, T>;
    fn mul(self, rhs: &Matrix<M, K, T>) -> Self::Output {
        assert!(M > 0);
        Matrix::new(|i, k| (0 .. M).map(|j| self[(i, j)].clone() * rhs[(j, k)].clone() ).reduce(|x, y| x + y ).expect("M > 0") )
    }
}
impl<const N: usize, const M: usize, T: std::fmt::Debug> std::fmt::Debug for Matrix<N, M, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::*;
        if N == 0 {
            return f.write_str("[]");
        }
        let a = (0 .. N).map(|i| (0 .. M).map(|j| format!("{:?}", self[(i, j)]) ).collect() ).collect::<Vec<Vec<_>>>();
        let col_ws = (0 .. M).map(|j| (0 .. N).map(|i| a[i][j].len() ).max().unwrap() ).collect::<Vec<_>>();
        for i in 0 .. N {
            if i == 0 {
                f.write_char('[')?;
            } else {
                f.write_char(' ')?;
            }
            for j in 0 .. M {
                f.write_str(&a[i][j])?;
                f.write_str(&" ".repeat(col_ws[j] - a[i][j].len()))?;
                if j == M - 1 {
                    if i == N - 1 {
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
```
