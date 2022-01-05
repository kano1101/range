extern crate chrono;
extern crate rstest;

use chrono::prelude::*;
use std::mem::replace;

// startもendも含む期間を提供する
pub struct Range {
    start: NaiveDate,
    end: NaiveDate,
}

impl Range {
    pub fn new(date1: &str, date2: &str) -> Self {
        let mut end = NaiveDate::parse_from_str(date1, "%Y-%m-%d").unwrap();
        let mut start = NaiveDate::parse_from_str(date2, "%Y-%m-%d").unwrap();
        if end < start {
            start = replace(&mut end, start);
        }
        Self {
            start: start,
            end: end,
        }
    }
    pub fn start(&self) -> String {
        self.start.to_string()
    }
    pub fn end(&self) -> String {
        self.end.to_string()
    }
    pub fn elapsed_days(&self) -> i64 {
        (self.end - self.start).num_days()
    }
    pub fn between(&self, date: &str) -> bool {
        let when = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
        (self.start <= when) && (when <= self.end)
    }
    pub fn start_before(&self, date: &str) -> bool {
        let when = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
        when < self.start
    }
    pub fn end_after(&self, date: &str) -> bool {
        let when = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
        self.end < when
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    #[test]
    fn 初期動作確認() {
        assert_eq!(1, 1);
    }
    #[test]
    fn 購入期間オブジェクトのテスト() {
        use super::Range;
        let range = Range::new("2021-10-01", "2021-04-01");
        assert_eq!(range.start(), "2021-04-01");
        assert_eq!(range.end(), "2021-10-01");
        assert_eq!(range.elapsed_days(), 7 * 26 + 1); // 経過日数は26週と1日
    }
    #[test]
    fn 購入期間オブジェクト生成引数順序逆のテスト() {
        use super::Range;
        let range = Range::new("2021-04-01", "2021-10-01");
        assert_eq!(range.start(), "2021-04-01");
        assert_eq!(range.end(), "2021-10-01");
        assert_eq!(range.elapsed_days(), 7 * 26 + 1); // 経過日数は26週と1日
    }
    #[test]
    fn 期間内を正しくチェックできるか調べるテスト() {
        use super::Range;
        let range = Range::new("2021-04-01", "2021-10-01");
        assert_eq!(range.between("2021-03-31"), false);
        assert_eq!(range.between("2021-04-01"), true);
        assert_eq!(range.between("2021-07-01"), true);
        assert_eq!(range.between("2021-10-01"), true);
        assert_eq!(range.between("2021-10-02"), false);
    }
    #[test]
    fn 期間外を正しくチェックできるか調べるテスト() {
        use super::Range;
        let range = Range::new("2021-04-01", "2021-10-01");
        assert_eq!(range.start_before("2021-03-31"), true);
        assert_eq!(range.start_before("2021-04-01"), false);
        assert_eq!(range.end_after("2021-10-01"), false);
        assert_eq!(range.end_after("2021-10-02"), true);
    }
}
