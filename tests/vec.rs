#[cfg(feature = "vec-v1")]
mod v1 {
    use musutils::vec::{get_page, print_json_as_table};
    use serde_json::json;

    #[test]
    fn get_page_first() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(get_page(&v, 0, 2), vec![1, 2]);
    }

    #[test]
    fn get_page_middle() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(get_page(&v, 1, 2), vec![3, 4]);
    }

    #[test]
    fn get_page_last_partial() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(get_page(&v, 2, 2), vec![5]);
    }

    #[test]
    fn get_page_out_of_range() {
        let v = vec![1, 2, 3];
        assert!(get_page(&v, 10, 2).is_empty());
    }

    #[test]
    fn get_page_empty_input() {
        let v: Vec<i32> = Vec::new();
        assert!(get_page(&v, 0, 10).is_empty());
    }

    #[test]
    fn get_page_zero_count() {
        let v = vec![1, 2, 3];
        assert!(get_page(&v, 0, 0).is_empty());
    }

    #[test]
    fn get_page_count_larger_than_len() {
        let v = vec![1, 2, 3];
        assert_eq!(get_page(&v, 0, 100), vec![1, 2, 3]);
    }

    #[test]
    fn get_page_clones() {
        let v = vec![String::from("a"), String::from("b")];
        let page = get_page(&v, 0, 1);
        assert_eq!(page, vec![String::from("a")]);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn print_empty_smoke() {
        print_json_as_table(&[]);
    }

    #[test]
    fn print_non_object_smoke() {
        let data = vec![json!("hello")];
        print_json_as_table(&data);
    }

    #[test]
    fn print_objects_smoke() {
        let data = vec![json!({"a": 1, "b": "x"}), json!({"a": 2})];
        print_json_as_table(&data);
    }
}

#[cfg(feature = "vec-v2")]
mod v2 {
    use musutils::vec::{
        PageError, TableError, get_page, json_as_table, page, print_json_as_table,
    };
    use serde_json::json;

    #[test]
    fn get_page_basic() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(get_page(&v, 0, 2), vec![1, 2]);
        assert_eq!(get_page(&v, 1, 2), vec![3, 4]);
        assert_eq!(get_page(&v, 2, 2), vec![5]);
    }

    #[test]
    fn get_page_out_of_range() {
        let v = vec![1, 2, 3];
        assert!(get_page(&v, 100, 10).is_empty());
    }

    #[test]
    fn get_page_empty_input() {
        let v: Vec<i32> = Vec::new();
        assert!(get_page(&v, 0, 1).is_empty());
    }

    #[test]
    fn get_page_zero_count() {
        let v = vec![1, 2, 3];
        assert!(get_page(&v, 0, 0).is_empty());
    }

    #[test]
    fn get_page_saturating_mul() {
        let v = vec![1, 2, 3];
        let r = get_page(&v, u32::MAX, u32::MAX);
        assert!(r.is_empty());
    }

    #[test]
    fn page_one_based() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(page(&v, 1, 2).unwrap(), vec![1, 2]);
        assert_eq!(page(&v, 2, 2).unwrap(), vec![3, 4]);
        assert_eq!(page(&v, 3, 2).unwrap(), vec![5]);
    }

    #[test]
    fn page_zero_returns_empty() {
        let v = vec![1, 2, 3];
        assert!(page(&v, 0, 2).unwrap().is_empty());
    }

    #[test]
    fn page_zero_per_page_err() {
        let v = vec![1, 2, 3];
        let err = page(&v, 1, 0).unwrap_err();
        assert!(matches!(err, PageError::ZeroPerPage));
        assert!(err.to_string().contains("per_page"));
    }

    #[test]
    fn page_out_of_range_empty() {
        let v = vec![1, 2, 3];
        assert!(page(&v, 100, 2).unwrap().is_empty());
    }

    #[test]
    fn page_empty_input() {
        let v: Vec<i32> = Vec::new();
        assert!(page(&v, 1, 5).unwrap().is_empty());
    }

    #[test]
    fn page_per_page_larger_than_len() {
        let v = vec![1, 2, 3];
        assert_eq!(page(&v, 1, 100).unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn json_as_table_empty_err() {
        let err = json_as_table(&[]).unwrap_err();
        assert!(matches!(err, TableError::Empty));
        assert!(err.to_string().contains("no data"));
    }

    #[test]
    fn json_as_table_no_fields_err() {
        let data = vec![json!("x"), json!(42)];
        let err = json_as_table(&data).unwrap_err();
        assert!(matches!(err, TableError::NoFields));
        assert!(err.to_string().contains("no fields"));
    }

    #[test]
    fn json_as_table_basic() {
        let data = vec![json!({"a": 1, "b": "x"}), json!({"a": 22, "b": "yy"})];
        let t = json_as_table(&data).unwrap();
        assert!(t.contains("a"));
        assert!(t.contains("b"));
        assert!(t.contains("22"));
        assert!(t.contains("yy"));
    }

    #[test]
    fn json_as_table_union_of_keys() {
        let data = vec![json!({"a": 1}), json!({"b": 2})];
        let t = json_as_table(&data).unwrap();
        let header = t.lines().next().unwrap();
        assert!(header.contains('a'));
        assert!(header.contains('b'));
        assert!(t.contains("N/A"));
    }

    #[test]
    fn json_as_table_mixed_values() {
        let data = vec![json!({"n": 5, "b": true, "z": null, "s": "hi"})];
        let t = json_as_table(&data).unwrap();
        assert!(t.contains('5'));
        assert!(t.contains("true"));
        assert!(t.contains("null"));
        assert!(t.contains("hi"));
    }

    #[test]
    fn json_as_table_unicode_keys() {
        let data = vec![json!({"имя": "значение"}), json!({"имя": "другое"})];
        let t = json_as_table(&data).unwrap();
        let mut lines = t.lines();
        let header = lines.next().unwrap();
        let sep = lines.next().unwrap();
        assert_eq!(header.chars().count(), sep.chars().count());
        assert!(header.contains("имя"));
        assert!(t.contains("значение"));
    }

    #[test]
    fn json_as_table_separator_lengths_match() {
        let data = vec![json!({"col": "value"})];
        let t = json_as_table(&data).unwrap();
        let lines: Vec<&str> = t.lines().collect();
        assert!(lines.len() >= 4);
        assert_eq!(lines[0].chars().count(), lines[1].chars().count());
        assert_eq!(
            lines[1].chars().count(),
            lines[lines.len() - 1].chars().count()
        );
    }

    #[test]
    fn json_as_table_skips_non_object_rows() {
        let data = vec![json!({"a": 1}), json!("skip"), json!({"a": 2})];
        let t = json_as_table(&data).unwrap();
        assert!(!t.contains("skip"));
        assert!(t.contains('1'));
        assert!(t.contains('2'));
    }

    #[test]
    fn print_empty_smoke() {
        print_json_as_table(&[]);
    }

    #[test]
    fn print_no_fields_smoke() {
        print_json_as_table(&[json!(1)]);
    }

    #[test]
    fn print_objects_smoke() {
        print_json_as_table(&[json!({"a": 1})]);
    }
}
