#[deprecated(note = "use `Option::unwrap_or_else` or `Option::unwrap_or` instead")]
pub fn deoption<T, F>(opt: Option<T>, default_fn: F, found: &mut bool) -> T
where
    F: FnOnce() -> T,
{
    match opt {
        Some(val) => {
            *found = true;
            val
        }
        None => {
            *found = false;
            default_fn()
        }
    }
}
