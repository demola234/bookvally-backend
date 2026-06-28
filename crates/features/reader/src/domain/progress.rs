use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Progress {
    pub library_item_id: Uuid,
    pub current_page: i32,
    pub total_pages: Option<i32>,
    pub current_locator: Option<String>,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ProgressError {
    #[error("page {0} exceeds total pages {1}")]
    PageExceedsTotal(i32, i32),
    #[error("page must not be negative")]
    NegativePage,
}

impl Progress {
    pub fn new(library_item_id: Uuid, total_pages: Option<i32>) -> Self {
        Self {
            library_item_id,
            current_page: 0,
            total_pages,
            current_locator: None,
        }
    }

    pub fn set_page(&mut self, page: i32) -> Result<(), ProgressError> {
        if page < 0 {
            return Err(ProgressError::NegativePage);
        }
        if let Some(total) = self.total_pages {
            if page > total {
                return Err(ProgressError::PageExceedsTotal(page, total));
            }
        }
        self.current_page = page;
        Ok(())
    }

    pub fn set_locator(&mut self, locator: impl Into<String>) {
        self.current_locator = Some(locator.into());
    }

    pub fn pct(&self) -> Decimal {
        match self.total_pages {
            Some(total) if total > 0 => {
                (Decimal::from(self.current_page) / Decimal::from(total) * dec!(100)).min(dec!(100))
            }
            _ => Decimal::ZERO,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.total_pages
            .map(|t| t > 0 && self.current_page >= t)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item() -> Uuid {
        Uuid::new_v4()
    }

    #[test]
    fn set_page_within_bounds_ok() {
        let mut p = Progress::new(item(), Some(300));
        p.set_page(150).unwrap();
        assert_eq!(p.current_page, 150);
    }

    #[test]
    fn set_page_to_last_page_ok() {
        let mut p = Progress::new(item(), Some(300));
        p.set_page(300).unwrap();
        assert_eq!(p.current_page, 300);
    }

    #[test]
    fn set_page_exceeds_total_is_err() {
        let mut p = Progress::new(item(), Some(300));
        let err = p.set_page(301).unwrap_err();
        assert_eq!(err, ProgressError::PageExceedsTotal(301, 300));
        assert_eq!(p.current_page, 0);
    }

    #[test]
    fn set_negative_page_is_err() {
        let mut p = Progress::new(item(), Some(300));
        assert_eq!(p.set_page(-1).unwrap_err(), ProgressError::NegativePage);
    }

    #[test]
    fn set_page_without_total_has_no_upper_bound() {
        let mut p = Progress::new(item(), None);
        p.set_page(9999).unwrap();
        assert_eq!(p.current_page, 9999);
    }

    #[test]
    fn pct_is_zero_on_fresh_progress() {
        let p = Progress::new(item(), Some(200));
        assert_eq!(p.pct(), dec!(0));
    }

    #[test]
    fn pct_is_50_at_halfway() {
        let mut p = Progress::new(item(), Some(200));
        p.set_page(100).unwrap();
        assert_eq!(p.pct(), dec!(50));
    }

    #[test]
    fn pct_is_100_at_last_page() {
        let mut p = Progress::new(item(), Some(200));
        p.set_page(200).unwrap();
        assert_eq!(p.pct(), dec!(100));
    }

    #[test]
    fn pct_is_zero_when_total_pages_unknown() {
        let p = Progress::new(item(), None);
        assert_eq!(p.pct(), dec!(0));
    }

    #[test]
    fn pct_is_zero_when_total_pages_is_zero() {
        let p = Progress::new(item(), Some(0));
        assert_eq!(p.pct(), dec!(0));
    }

    #[test]
    fn pct_caps_at_100() {
        let mut p = Progress::new(item(), Some(100));
        p.current_page = 120;
        assert_eq!(p.pct(), dec!(100));
    }

    #[test]
    fn is_finished_when_at_last_page() {
        let mut p = Progress::new(item(), Some(300));
        p.set_page(300).unwrap();
        assert!(p.is_finished());
    }

    #[test]
    fn is_not_finished_one_page_before_end() {
        let mut p = Progress::new(item(), Some(300));
        p.set_page(299).unwrap();
        assert!(!p.is_finished());
    }

    #[test]
    fn is_not_finished_when_total_unknown() {
        let mut p = Progress::new(item(), None);
        p.set_page(500).unwrap();
        assert!(!p.is_finished());
    }

    #[test]
    fn is_not_finished_when_total_is_zero() {
        let p = Progress::new(item(), Some(0));
        assert!(!p.is_finished());
    }

    #[test]
    fn set_locator_is_independent_of_page() {
        let mut p = Progress::new(item(), Some(300));
        p.set_locator("epubcfi(/6/4[chap01]!/4/2/1:0)");
        assert_eq!(p.current_page, 0);
        assert!(p.current_locator.is_some());
    }
}
