/// Return the edge cursor pair for the current page.
///
/// The returned tuple is ordered as `(newer_cursor, older_cursor)`.
pub fn edge_cursors<T, K, F>(items: &[T], is_newer: bool, key: F) -> Option<(K, K)>
where
    K: Copy,
    F: Fn(&T) -> K,
{
    let (Some(first), Some(last)) = (items.first(), items.last()) else {
        return None;
    };

    let first_key = key(first);
    let last_key = key(last);

    if is_newer {
        Some((last_key, first_key))
    } else {
        Some((first_key, last_key))
    }
}

/// Reverse items when the page was fetched in the `Newer` direction.
pub fn reverse_if_newer<T>(items: &mut [T], is_newer: bool) {
    if is_newer {
        items.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::{edge_cursors, reverse_if_newer};

    #[derive(Clone, Copy)]
    struct Item {
        id: u32,
    }

    #[test]
    fn edge_cursors_returns_none_for_empty_items() {
        let items: Vec<Item> = vec![];
        assert_eq!(edge_cursors(&items, false, |item| item.id), None);
    }

    #[test]
    fn edge_cursors_uses_first_as_newer_and_last_as_older_when_direction_is_older() {
        let items = vec![Item { id: 30 }, Item { id: 20 }, Item { id: 10 }];
        assert_eq!(edge_cursors(&items, false, |item| item.id), Some((30, 10)));
    }

    #[test]
    fn edge_cursors_uses_last_as_newer_and_first_as_older_when_direction_is_newer() {
        let items = vec![Item { id: 10 }, Item { id: 20 }, Item { id: 30 }];
        assert_eq!(edge_cursors(&items, true, |item| item.id), Some((30, 10)));
    }

    #[test]
    fn reverse_if_newer_reverses_only_when_direction_is_newer() {
        let mut older_items = vec![3, 2, 1];
        reverse_if_newer(&mut older_items, false);
        assert_eq!(older_items, vec![3, 2, 1]);

        let mut newer_items = vec![1, 2, 3];
        reverse_if_newer(&mut newer_items, true);
        assert_eq!(newer_items, vec![3, 2, 1]);
    }
}
