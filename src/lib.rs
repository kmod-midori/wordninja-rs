use lazy_static::lazy_static;
use std::collections::HashMap;
use std::{borrow::Cow, cmp::min};

lazy_static! {
    pub static ref DEFAULT_MODEL: LanguageModel<'static> =
        LanguageModel::new_model(include_str!("../wordninja_words.txt"));
}

#[derive(Debug, Clone)]
pub struct LanguageModel<'s> {
    wordcost: HashMap<&'s str, f64>,
    maxword: usize,
}

impl<'s> LanguageModel<'s> {
    pub fn new(words: &[&'s str]) -> Self {
        let count = words.len();
        let count_ln = (count as f64).ln();
        let mut wordcost = HashMap::with_capacity(count);
        let mut maxword = 0;

        for (i, word) in words.iter().enumerate() {
            // Build a cost dictionary, assuming Zipf's law and cost = -math.log(probability).
            let cost = (((i + 1) as f64) * count_ln).ln();
            wordcost.insert(*word, cost);

            if word.len() > maxword {
                maxword = word.len();
            }
        }

        Self { wordcost, maxword }
    }

    pub fn new_model(model: &'s str) -> Self {
        let words: Vec<&str> = model.lines().filter(|s| !s.is_empty()).collect();
        Self::new(&words)
    }

    pub fn split<'a>(&'a self, s: &'a str) -> impl Iterator<Item = Cow<'a, str>> {
        s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '\''))
            .filter(|s| !s.is_empty())
            .flat_map(move |s| self.split_part(s))
    }

    fn split_part<'a>(&self, s: &'a str) -> impl Iterator<Item = Cow<'a, str>> {
        // Build the cost array.
        let mut cost = Vec::with_capacity(s.len() + 1);
        cost.push(0.0);
        for i in 1..s.len() + 1 {
            let (c, _) = self.best_match(s, &cost, i);
            cost.push(c);
        }

        // Backtrack to recover the minimal-cost string.
        let mut out: Vec<Cow<str>> = vec![];
        let mut i = s.len();

        while i > 0 {
            let (c, k) = self.best_match(s, &cost, i);
            assert_eq!(c, cost[i]);
            // Apostrophe and digit handling (added by Genesys)
            let mut new_token = true;
            let token = &s[i - k..i];

            if token != "'" {
                // ignore a lone apostrophe
                if !out.is_empty() {
                    // re-attach split 's and split digits
                    let last_out = out.iter_mut().last().unwrap();
                    let is_s = *last_out == "'s";
                    // digit followed by digit
                    let digit_follow = s.chars().nth(i - 1).unwrap().is_ascii_digit()
                        && last_out.chars().next().unwrap().is_ascii_digit();
                    if is_s || digit_follow {
                        *last_out = [token, last_out].concat().into();
                        new_token = false;
                    }
                }
            }
            // (End of Genesys addition)

            if new_token {
                out.push(token.into());
            }

            i -= k;
        }

        out.into_iter().rev()
    }

    fn best_match(&self, s: &str, cost: &[f64], i: usize) -> (f64, usize) {
        let back = min(self.maxword, i);

        cost[i - back..i]
            .iter()
            .rev()
            .enumerate()
            .map(|(k, c)| {
                let lower = s[i - k - 1..i].to_ascii_lowercase();
                let match_cost = c + self.wordcost.get(&lower[..]).unwrap_or(&f64::MAX);
                (match_cost, k + 1)
            })
            .min_by(|(x, _), (y, _)| x.partial_cmp(y).expect("Tried to compare a NaN"))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(
            DEFAULT_MODEL.split("derekanderson").collect::<Vec<_>>(),
            vec!["derek", "anderson"]
        );
        assert_eq!(
            DEFAULT_MODEL.split("DEREKANDERSON").collect::<Vec<_>>(),
            vec!["DEREK", "ANDERSON"]
        );
    }

    #[test]
    fn with_underscores_etc() {
        let cmp = vec!["derek", "anderson"];
        assert_eq!(
            DEFAULT_MODEL.split("derek anderson").collect::<Vec<_>>(),
            cmp
        );
        assert_eq!(
            DEFAULT_MODEL.split("derek-anderson").collect::<Vec<_>>(),
            cmp
        );
        assert_eq!(
            DEFAULT_MODEL.split("derek_anderson").collect::<Vec<_>>(),
            cmp
        );
        assert_eq!(
            DEFAULT_MODEL.split("derek/anderson").collect::<Vec<_>>(),
            cmp
        );
    }

    #[test]
    fn digits() {
        assert_eq!(
            DEFAULT_MODEL.split("win32intel").collect::<Vec<_>>(),
            vec!["win", "32", "intel"]
        );
    }

    #[test]
    fn apostrophes() {
        assert_eq!(
            DEFAULT_MODEL
                .split("that'sthesheriff'sbadge")
                .collect::<Vec<_>>(),
            vec!["that's", "the", "sheriff's", "badge"]
        );
    }

    #[test]
    fn custom() {
        let model = LanguageModel::new_model(include_str!("../test_lang.txt"));
        assert_eq!(
            model.split("derek").collect::<Vec<_>>(),
            vec!["der", "ek"]
        );
    }
}
