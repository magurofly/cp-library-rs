# Aho—Corasick

複数のパターンを文字列から検索する。未 Verify

## 使い方

- `let aho_corasick = AhoCorasick::new(letters, patterns)`
  - `patterns` を検索するために初期化する。
  - `letters: usize`: 文字種
  - `patterns: &[Vec<usize>]`: パターン（複数）、各パターンは文字（`letters` 未満の非負整数）からなる `Vec`
- `let result = aho_corasick.find(string)`
  - `string` に含まれるパターンを検索し、含まれるパターンと位置を列挙する。
  - `string: &[usize]`: `letters` 未満の非負整数からなるスライス
  - `result: Vec<(usize, usize)>`: `(マッチ位置, パターン番号)` からなる `Vec`
- `let dfa = aho_corasick.to_dfa()`
  - DFA に変換する。
  - `dfa: Vec<(Vec<usize>, Vec<usize>)>`
    - `dfa[state].0[letter] = next_state`
    - `dfa[state].1 = patterns`

## コード

```rs
use aho_corasick::*;
pub mod aho_corasick {
  #[derive(Clone, Debug)]
  pub struct PMANode {
    children: Vec<Option<usize>>,
    failure: usize,
    /// vec![(len, id)]
    patterns: Vec<(usize, usize)>,
  }
  impl PMANode {
    pub fn new(letters: usize) -> Self {
      Self { children: vec![None; letters], failure: 0, patterns: vec![] }
    }
  }

  #[derive(Clone, Debug)]
  /// Aho-Corasick 法によるオートマトンを隣接リスト形式で返す
  /// `dfa[state] = (vec![(next_state, letter)], match_count)`
  pub struct AhoCorasick {
    letters: usize,
    pub pma: Vec<PMANode>,
  }
  impl AhoCorasick {
    pub fn new(letters: usize, patterns: &[Vec<usize>]) -> Self {
      assert!(patterns.iter().all(|pattern| pattern.iter().all(|&letter| letter < letters ) ));

      // Trie を構築
      let mut pma = vec![PMANode::new(letters)]; // (vec![next_state], vec![matched_pattern])
      for (id, pattern) in patterns.iter().enumerate() {
        let mut state = 0;
        for &c in pattern {
          state = pma[state].children[c].unwrap_or_else(|| {
            let next_state = pma.len();
            pma[state].children[c] = Some(next_state);
            pma.push(PMANode::new(letters));
            next_state
          });
        }
        pma[state].patterns.push((pattern.len(), id));
      }

      let mut queue = std::collections::VecDeque::new();
      // 1 文字パターンの failure は根とする
      for c in 0 .. letters {
        if let Some(u) = pma[0].children[c] {
          pma[u].failure = 0;
          queue.push_back(u);
        }
      }
      // BFS して failure を設定
      while let Some(u) = queue.pop_front() {
        for c in 0 .. letters {
          if let Some(v) = pma[u].children[c] {
            queue.push_back(v);
            let mut f = pma[u].failure;
            while f != 0 && pma[f].children[c].is_none() {
              f = pma[f].failure;
            }
            pma[v].failure = pma[f].children[c].unwrap_or(0);
            let mut failure_patterns = pma[pma[v].failure].patterns.to_vec();
            pma[v].patterns.append(&mut failure_patterns);
          }
        }
      }

      Self { letters, pma }
    }

    /// `haystack` 中に含まれるパターンを列挙する
    /// 返り値: `vec![(位置, パターン番号)]`
    pub fn find(&self, haystack: &[usize]) -> Vec<(usize, usize)> {
      let mut result = vec![];
      let mut state = 0;
      for offset in 0 .. haystack.len() {
        let c = haystack[offset];
        assert!(c < self.letters);
        while state != 0 && self.pma[state].children[c].is_none() {
          state = self.pma[state].failure;
        }
        state = self.pma[state].children[c].unwrap_or(0);
        for &(len, id) in &self.pma[state].patterns {
          result.push((offset + 1 - len, id));
        }
      }
      result
    }

    /// DFA に変換する
    /// `dfa[state] = (vec![next_state], vec![pattern_id])`
    pub fn to_dfa(&self) -> Vec<(Vec<usize>, Vec<usize>)> {
      let mut dfa = vec![];
      for state in 0 .. self.pma.len() {
        let next_states = (0 .. self.letters).map(|c| {
          let mut f = state;
          while f != 0 && self.pma[f].children[c].is_none() {
            f = self.pma[f].failure;
          }
          self.pma[f].children[c].unwrap_or(0)
        }).collect();
        let pattern_ids = self.pma[state].patterns.iter().map(|&(_, pattern_id)| pattern_id ).collect();
        dfa.push((next_states, pattern_ids));
      }
      dfa
    }
  }
}
```
