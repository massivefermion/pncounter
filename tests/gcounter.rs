#[cfg(test)]
mod tests {
    use crdt::gcounter::GCounter;

    #[test]
    fn defaults_to_zero() {
        let c = GCounter::new(5, 10);
        assert_eq!(c.query(), 0);
    }

    #[test]
    fn add() {
        let mut c = GCounter::new(3, 10);
        c.add(9);
        assert_eq!(c.query(), 9);
    }

    #[test]
    fn merge() {
        let mut c1 = GCounter::new(5, 10);
        let mut c2 = GCounter::new(7, 10);

        c1.add(5);
        c2.add(8);

        c1.merge(c2);

        assert_eq!(c1.query(), 13);
    }
}
