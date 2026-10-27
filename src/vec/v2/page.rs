use super::PageError;

pub fn page<T: Clone>(items: &[T], page: u32, per_page: u32) -> Result<Vec<T>, PageError> {
    if per_page == 0 {
        return Err(PageError::ZeroPerPage);
    }
    if page == 0 {
        return Ok(Vec::new());
    }
    let per_page = per_page as usize;
    let skip = (page as usize - 1).saturating_mul(per_page);
    Ok(items.iter().skip(skip).take(per_page).cloned().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_one_based() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(page(&v, 1, 2).unwrap(), vec![1, 2]);
        assert_eq!(page(&v, 3, 2).unwrap(), vec![5]);
        assert_eq!(page(&v, 0, 2).unwrap(), Vec::<i32>::new());
    }

    #[test]
    fn page_zero_per_page_err() {
        let v = vec![1];
        assert!(page(&v, 1, 0).is_err());
    }
}
