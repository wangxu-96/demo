#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3, 4, 5];
        let mut v1_it = v1.iter();
        assert_eq!(v1_it.next(), Some(&1));
        assert_eq!(v1_it.next(), Some(&2));
        assert_eq!(v1_it.next(), Some(&3));

        let x: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(x, vec![2, 3, 4, 5, 6]);
    }
}