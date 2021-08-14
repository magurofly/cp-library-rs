use acl_convolution::*;
use acl_modint::*;
use std::marker::PhantomData;
use num_traits::*;
use super::*;

pub trait Convolution<T> {
  fn convolution(a: &[T], b: &[T]) -> Vec<T>;
}

#[derive(Debug, Clone, Default)]
pub struct ConvolutionStatic<M>(PhantomData<M>);
impl<M: Modulus> Convolution<StaticModInt<M>> for ConvolutionStatic<M> {
  fn convolution(a: &[StaticModInt<M>], b: &[StaticModInt<M>]) -> Vec<StaticModInt<M>> {
    if M::HINT_VALUE_IS_PRIME && (M::VALUE - 1).trailing_zeros() >= 20 {
      convolution(a, b)
    } else {
      let a = a.iter().map(|x| x.val() as i64).collect::<Vec<_>>();
      let b = b.iter().map(|x| x.val() as i64).collect::<Vec<_>>();
      convolution_i64(&a, &b).into_iter().map(|x| (x % <StaticModInt<M>>::modulus() as i64).into()).collect::<Vec<_>>()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct ConvolutionDynamic<I>(PhantomData<I>);
impl<I: Id> Convolution<DynamicModInt<I>> for ConvolutionDynamic<I> {
  fn convolution(a: &[DynamicModInt<I>], b: &[DynamicModInt<I>]) -> Vec<DynamicModInt<I>> {
    let a = a.iter().map(|x| x.val() as i64).collect::<Vec<_>>();
    let b = b.iter().map(|x| x.val() as i64).collect::<Vec<_>>();
    let c = convolution_i64(&a, &b).into_iter().map(|x| (x % <DynamicModInt<I>>::modulus() as i64).into()).collect::<Vec<_>>();
    c
  }
}

#[derive(Debug, Clone, Default)]
pub struct ConvolutionFloat<F>(PhantomData<F>);
impl<F: Float + FloatConst> Convolution<F> for ConvolutionFloat<F> {
  // https://ei1333.github.io/luzhiled/snippets/math/fast-fourier-transform.html
  fn convolution(a: &[F], b: &[F]) -> Vec<F> {
    let n = a.len() + b.len() - 1;
    let m = n.next_power_of_two();
    let mut a = a.to_vec();
    a.resize(m, F::zero());
    let mut b = b.to_vec();
    b.resize(m, F::zero());
    let mut x = rfft(&a);
    let y = rfft(&b);
    for (z, w) in x.iter_mut().zip(y) {
      *z = *z * w;
    }
    ifft_inplace(&mut x);
    x[.. n].iter().map(|z| z.re).collect()
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_convolution_mod1000000007() {
    use super::*;

    fn conv(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
      flat_vec(<ConvolutionStatic<Mod1000000007>>::convolution(&cast_vec(a), &cast_vec(b)))
    }

    assert_eq!(conv(vec![1, 2, 3, 4], vec![5, 6, 7, 8, 9]), vec![5, 16, 34, 60, 70, 70, 59, 36]);
    assert_eq!(conv(vec![10000000], vec![10000000]), vec![999300007]);
  }

  fn cast_vec<T, U: From<T>>(a: Vec<T>) -> Vec<U> {
    a.into_iter().map(U::from).collect::<Vec<_>>()
  }

  fn flat_vec<M: acl_modint::ModIntBase>(a: Vec<M>) -> Vec<u32> {
    a.into_iter().map(M::val).collect::<Vec<_>>()
  }

  #[test]
  fn test_convolution_float() {
    use super::*;

    fn conv(a: Vec<i64>, b: Vec<i64>) -> Vec<i64> {
      let a = a.into_iter().map(|x| x as f64).collect::<Vec<_>>();
      let b = b.into_iter().map(|x| x as f64).collect::<Vec<_>>();
      let c = ConvolutionFloat::convolution(&a, &b);
      c.into_iter().map(|x| x.round() as i64).collect::<Vec<_>>()
    }

    assert_eq!(conv(vec![1, 2, 3, 4], vec![1, 2, 4, 8]), vec![1, 4, 11, 26, 36, 40, 32])
  }
}