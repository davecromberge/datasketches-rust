use datasketches::cpc::CpcSketch;
use datasketches::hash::hash128;
use datasketches::hash::value::raw_bytes;
use datasketches::theta::ThetaSketchBuilder;

const SEED: u64 = 9001;

#[test]
fn update_hashed_matches_update() {
    for n in [0usize, 1, 100, 50_000] {
        let keys: Vec<String> = (0..n).map(|i| format!("user-{i}")).collect();
        let (mut ta, mut tb) = (ThetaSketchBuilder::default().build().unwrap(), ThetaSketchBuilder::default().build().unwrap());
        let (mut ca, mut cb) = (CpcSketch::new(11).unwrap(), CpcSketch::new(11).unwrap());
        for k in &keys {
            ta.update(raw_bytes::from_str(k));
            ca.update(raw_bytes::from_str(k));
            let h = hash128(raw_bytes::from_str(k), SEED);
            tb.update_hashed(h);
            cb.update_hashed(h);
        }
        assert_eq!(ta.compact(true).serialize(), tb.compact(true).serialize(), "theta n={n}");
        assert_eq!(ca.serialize(), cb.serialize(), "cpc n={n}");
    }
}
