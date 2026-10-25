pub fn get_page<T: Clone>(items: &[T], page: u32, count: u32) -> Vec<T> {
    let page = page as usize;
    let count = count as usize;
    let skip = page.saturating_mul(count);
    items.iter().skip(skip).take(count).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_page_basic() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(get_page(&v, 0, 2), vec![1, 2]);
        assert_eq!(get_page(&v, 1, 2), vec![3, 4]);
        assert_eq!(get_page(&v, 2, 2), vec![5]);
    }
}
