#[cfg(test)]
mod tests {
    use crdt::pncounter::PNCounter;

    #[test]
    fn defaults_to_zero() {
        let c = PNCounter::new(5, 10);
        assert_eq!(c.query(), 0);
    }

    #[test]
    fn add() {
        let mut c = PNCounter::new(3, 10);
        c.add(9);
        assert_eq!(c.query(), 9);
    }

    #[test]
    fn subtract() {
        let mut c = PNCounter::new(8, 10);
        c.subtract(12);
        assert_eq!(c.query(), -12);
    }

    #[test]
    fn add_and_subtract() {
        let mut c = PNCounter::new(2, 10);

        c.add(5);
        c.subtract(7);

        assert_eq!(c.query(), -2);
    }

    #[test]
    fn merge() {
        let mut c1 = PNCounter::new(5, 10);
        let mut c2 = PNCounter::new(7, 10);

        c1.add(5);
        c2.subtract(8);

        c1.merge(c2);

        assert_eq!(c1.query(), -3);
    }
}
