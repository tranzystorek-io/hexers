mod aux;
mod hexes;
pub use hexes::{HexIterator, Hexes};

#[cfg(test)]
mod tests {
    use crate::hexes::HexIterator;

    #[test]
    fn hexes_iterates_over_an_array() {
        let data = [0xbe, 0xef];
        let it = data.iter().copied().hexed();

        let collected: String = it.collect();
        assert_eq!(collected.as_str(), "beef");
    }

    #[test]
    fn hexes_reverse_iterates_over_an_array() {
        let data = [0xbe, 0xef];
        let it = data.iter().copied().hexed().rev();

        let collected: String = it.collect();
        assert_eq!(collected.as_str(), "feeb");
    }

    #[test]
    fn hexes_transforms_ranges() {
        let range = 10..16;
        let it = range.hexed();

        let collected: String = it.collect();
        assert_eq!(collected.as_str(), "0a0b0c0d0e0f");
    }

    #[test]
    fn hexes_works_for_converted_strings() {
        let data = b"beef";
        let it = data.iter().copied().hexed();

        let collected: String = it.collect();
        assert_eq!(collected.as_str(), "62656566");
    }

    #[test]
    fn hexes_can_be_consumed_from_both_ends() {
        let data = [0xde, 0xad, 0xbe, 0xef];
        let mut it = data.iter().copied().hexed();

        let front = it.by_ref().take(3);
        let expected = "dea".chars();
        assert!(front.eq(expected));

        let expected_back = "feebd".chars();
        assert!(it.rev().eq(expected_back));
    }
}
