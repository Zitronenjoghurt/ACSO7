use crate::data::names::{FirstName, LastName, Region};
use crate::world::colonist::Sex;
use strum::IntoEnumIterator;

const MIX_CHANCE: f64 = 0.08;

fn weighted<T: Copy>(rng: &mut fastrand::Rng, items: &[(T, u32)]) -> T {
    let total: u64 = items.iter().map(|(_, w)| *w as u64).sum();
    let mut roll = rng.u64(0..total);
    for (item, w) in items {
        let w = *w as u64;
        if roll < w {
            return *item;
        }
        roll -= w;
    }
    items[items.len() - 1].0
}

pub fn pick_region(rng: &mut fastrand::Rng) -> Region {
    let total: u64 = Region::iter().map(|r| r.weight()).sum();
    let mut roll = rng.u64(0..total);
    for region in Region::iter() {
        let w = region.weight();
        if roll < w {
            return region;
        }
        roll -= w;
    }
    Region::Anglo
}

pub fn first_name(rng: &mut fastrand::Rng, region: Region, sex: Sex) -> FirstName {
    match sex {
        Sex::Female => weighted(rng, region.female_names()),
        Sex::Male => weighted(rng, region.male_names()),
    }
}

pub fn last_name(rng: &mut fastrand::Rng, region: Region) -> LastName {
    weighted(rng, region.last_names())
}

pub fn generate(rng: &mut fastrand::Rng, sex: Sex) -> (FirstName, LastName) {
    let region = pick_region(rng);
    let first = first_name(rng, region, sex);
    let last_region = if rng.f64() < MIX_CHANCE {
        pick_region(rng)
    } else {
        region
    };
    (first, last_name(rng, last_region))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_non_empty_names() {
        let mut rng = fastrand::Rng::with_seed(42);
        for _ in 0..1000 {
            let (first, last) = generate(&mut rng, Sex::Female);
            assert!(!first.as_str().is_empty());
            assert!(!last.as_str().is_empty());
        }
    }

    #[test]
    fn every_region_has_a_full_roster() {
        for region in Region::iter() {
            assert!(!region.female_names().is_empty(), "{:?}", region);
            assert!(!region.male_names().is_empty(), "{:?}", region);
            assert!(!region.last_names().is_empty(), "{:?}", region);
        }
    }

    #[test]
    fn rmp_serializes_via_compact_index() {
        let name = FirstName::iter().next().unwrap();
        let bytes = rmp_serde::to_vec(&name).unwrap();
        assert!(bytes.len() <= 3);
        let back: FirstName = rmp_serde::from_slice(&bytes).unwrap();
        assert_eq!(name, back);
    }
}
