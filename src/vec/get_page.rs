pub fn get_page<T: Clone>(items: &[T], page: u32, count: u32) -> Vec<T> {
    let page = page as usize;
    let count = count as usize;

    let skip_amount = page * count;

    items
        .iter()
        .skip(skip_amount) // Пропускаем старые страницы
        .take(count)       // Берем элементы для текущей страницы
        .cloned()          // Клонируем их, чтобы вернуть новый Vec
        .collect()         // Собираем в Vec<T>
}
