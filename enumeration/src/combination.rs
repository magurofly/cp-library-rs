use number::*;
use number::int_like::IntLike;

/// 二項係数を求める O(r)
pub fn binomial<N: IntLike>(n: N, r: N) -> N {
  let mut x = N::get1();
  for i in 1 ..= r.as_usize() {
    x = x * N::from_usize(i) / N::from_usize(n.as_usize() - i + 1);
  }
  x
}

/// 重複組合せを求める O(r)
pub fn combination_with_repetition<N: IntLike>(n: N, r: N) -> N {
  binomial((n + r).sub1(), r)
}

/// 順列を求める O(n)
pub fn permutation<N: IntLike>(n: N, r: N) -> N {
  let n = n.as_usize();
  let r = r.as_usize();
  N::from_usize(factorial(n)) / N::from_usize(factorial(n - r))
}

pub fn montmort_list<N: Int>(n: N) -> Vec<N> {
  let mut a = vec![N::zero(), N::one()];
  a.reserve(n.as_usize() - 1);
  for i in 2 ..= n.as_usize() {
    a.push((n - i.cast()) * (a[i - 1] + a[i - 2]));
  }
  a
}

/// 完全順列（撹乱順列）を求める O(n)
pub fn montmort<N: Int>(n: N) -> N {
  montmort_list(n)[n.as_usize()]
}

/// カタラン数を求める O(n)
pub fn catalan<N: Int>(n: N) -> N {
  binomial(n + n, n) / n.add1()
}