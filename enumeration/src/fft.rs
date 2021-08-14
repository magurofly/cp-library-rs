// http://wwwa.pikara.ne.jp/okojisan/stockham/optimization1.html

use num_complex::*;
use num_traits::*;

pub fn rfft<T: Float + FloatConst>(x: &[T]) -> Vec<Complex<T>> {
  let mut y  = x.into_iter().map(|&a| Complex::new(a, T::zero())).collect::<Vec<_>>();
  fft_inplace(&mut y);
  y
}

pub fn rifft<T: Float + FloatConst>(x: &[T]) -> Vec<Complex<T>> {
  let mut y  = x.into_iter().map(|&a| Complex::new(a, T::zero())).collect::<Vec<_>>();
  ifft_inplace(&mut y);
  y
}

pub fn fft<T: Float + FloatConst>(x: &[Complex<T>]) -> Vec<Complex<T>> {
  let mut y  = x.to_vec();
  fft_inplace(&mut y);
  y
}

pub fn ifft<T: Float + FloatConst>(x: &[Complex<T>]) -> Vec<Complex<T>> {
  let mut y  = x.to_vec();
  ifft_inplace(&mut y);
  y
}

pub fn fft_inplace<T: Float + FloatConst>(x: &mut Vec<Complex<T>>) {
  let n = x.len();
  fft_internal(x, &mut vec![Complex::zero(); n], n, 1, 0);
  for z in x.iter_mut() {
    *z = *z / T::from(n).unwrap();
  }
}

pub fn ifft_inplace<T: Float + FloatConst>(x: &mut Vec<Complex<T>>) {
  let n = x.len();
  for z in x.iter_mut() {
    *z = z.conj();
  }
  fft_internal(x, &mut vec![Complex::zero(); n], n, 1, 0);
  for z in x.iter_mut() {
    *z = z.conj();
  }
}

fn fft_internal<T: Float + FloatConst>(x: &mut [Complex<T>], y: &mut [Complex<T>], n: usize, width: usize, xy: usize) {
  if n == 2 {
    let z = [x, y];
    for q in 0 .. width {
      let a = z[0][q];
      let b = z[0][q + width];
      z[xy][q] = a + b;
      z[xy][q + width] = a + b;
    }
  } else if n >= 4 { 
    let m = n / 2;
    let theta0 = T::PI() * T::from(2).unwrap() / T::from(n).unwrap();
    for p in 0 .. m {
      let theta = theta0 * T::from(p).unwrap();
      let wp = Complex::new(theta.cos(), -theta.sin());
      for q in 0 .. width {
        let a = x[q + width * p];
        let b = x[q + width * (p + m)];
        y[q + width * (2 * p)] = a + b;
        y[q + width * (2 * p + 1)] = (a - b) * wp;
      }
    }
    fft_internal(y, x, m, 2 * width, xy ^ 1);
  }
}