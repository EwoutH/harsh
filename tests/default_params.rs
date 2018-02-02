extern crate harsh;

use harsh::Harsh;

const TEST_CASES: [(&str, &[u64]); 14] = [
    ("gY", &[0]),
    ("jR", &[1]),
    ("R8ZN0", &[928_728]),
    ("o2fXhV", &[1, 2, 3]),
    ("jRfMcP", &[1, 0, 0]),
    ("jQcMcW", &[0, 0, 1]),
    ("gYcxcr", &[0, 0, 0]),
    ("gLpmopgO6", &[1_000_000_000_000]),
    ("lEW77X7g527", &[0x1F_FFFF_FFFF_FFFF]),
    (
        "BrtltWt2tyt1tvt7tJt2t1tD",
        &[5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
    ),
    (
        "G6XOnGQgIpcVcXcqZ4B8Q8B9y",
        &[10_000_000_000, 0, 0, 0, 999_999_999_999_999],
    ),
    (
        "5KoLLVL49RLhYkppOplM6piwWNNANny8N",
        &[0x1F_FFFF_FFFF_FFFF, 0x1F_FFFF_FFFF_FFFF, 0x1F_FFFF_FFFF_FFFF],
    ),
    (
        "BPg3Qx5f8VrvQkS16wpmwIgj9Q4Jsr93gqx",
        &[1_000_000_001, 1_000_000_002, 1_000_000_003, 1_000_000_004, 1_000_000_005],
    ),
    (
        "1wfphpilsMtNumCRFRHXIDSqT2UPcWf1hZi3s7tN",
        &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
        ],
    ),
];

#[test]
fn default_params() {
    let harsh = Harsh::default();

    for &(hash, values) in &TEST_CASES {
        assert_eq!(hash, harsh.encode(values).unwrap());
        assert_eq!(values, &harsh.decode(hash).unwrap()[..]);
    }
}
