use crate::pyrand::Random;

// ten digits after the decimal point
fn tendigits(value: f64) -> String {
    let mut result = format!("{:1.11}", value);
    result.truncate(12);
    result
}

fn gen_u32(seed: u32, iterations: u32) -> String {
    let mut r = Random::new();

    r.seed_u32(seed);

    for _ in 0..iterations {
        r.rand();
    }

    tendigits(r.rand())
}

#[test]
fn test_rand_seed_u32() {
    assert_eq!(gen_u32(0, 0), "0.8444218515");
    assert_eq!(gen_u32(654321, 0), "0.0657799204");
    assert_eq!(gen_u32(0, 987654), "0.7203062140");
    assert_eq!(gen_u32(123, 1000), "0.0638474579");
    assert_eq!(gen_u32(0xfffe, 5927), "0.5279272538");
    assert_eq!(gen_u32(0xffff, 5927), "0.7647091554");
    assert_eq!(gen_u32(0x10000, 5927), "0.8890962216");
}

fn gen_hex(seed: &str, iterations: u32) -> String {
    let mut r = Random::new();

    r.seed_hex_string(String::from(seed)).unwrap();

    for _ in 0..iterations {
        r.rand();
    }

    tendigits(r.rand())
}

#[test]
fn test_rand_seed_hex_string() {
    assert_eq!(gen_hex("1", 0), "0.1343642441");
    assert_eq!(gen_hex("12", 0), "0.1812648633");
    assert_eq!(gen_hex("123", 0), "0.6584375088");
    assert_eq!(gen_hex("1234", 0), "0.9281175899");
    assert_eq!(gen_hex("12345", 0), "0.3929596428");
    assert_eq!(gen_hex("123456", 0), "0.1217380432");
    assert_eq!(gen_hex("12345678", 0), "0.8608697203");
    assert_eq!(gen_hex("1234567890ab", 0), "0.9499690417");
    assert_eq!(gen_hex("1234567890abcdef", 0), "0.4333428902");
    assert_eq!(
        gen_hex("1234567890abcdef1234567890abcdef", 0),
        "0.4044561140"
    );
}

fn gen_u64(seed: u64, iterations: u32) -> String {
    let mut r = Random::new();

    r.seed_u64(seed);

    for _ in 0..iterations {
        r.rand();
    }

    tendigits(r.rand())
}

#[test]
fn test_rand_seed_u64() {
    assert_eq!(gen_u64(0xa37b3f09a188eu64, 12345), "0.6162433684");
    assert_eq!(gen_u64(0xffffffffffffffffu64, 999), "0.9009945166");
    assert_eq!(gen_u64(432153415134, 986), "0.7026873940");
}

fn gen_below(seed: u64, iterations: u32, below: u64) -> u64 {
    let mut r = Random::new();

    r.seed_u64(seed);

    for _ in 0..iterations {
        r.rand();
    }

    r.rand_below(below)
}

#[test]
fn test_rand_below() {
    assert_eq!(gen_below(117624834567, 5678, 2000), 1453);
    assert_eq!(gen_below(6513265496841, 4567, 0xfffffffd), 2688309836);
    assert_eq!(gen_below(65132495874231, 12288, 0xffffffff), 848139872);
    assert_eq!(gen_below(987651354, 16587, 0x100000000), 617983553);
    assert_eq!(gen_below(1684651512, 3486, 0x100000001), 3726269297);
    assert_eq!(
        gen_below(17209, 68133, 0xfffffffffffffffe),
        17889265393449113490
    );
    assert_eq!(
        gen_below(555555, 17009, 0xffffffffffffffff),
        14674416218734170714
    );
}

fn gen_bits(seed: u32, iterations: u32, bits: u32) -> Vec<u32> {
    let mut r = Random::new();

    r.seed_u32(seed);

    for _ in 0..iterations {
        r.rand();
    }

    r.rand_bits(bits)
}

#[test]
fn test_rand_bits() {
    assert_eq!(gen_bits(0, 0, 8), vec![216]);
    assert_eq!(gen_bits(0, 0, 32), vec![3626764237]);
    assert_eq!(gen_bits(0, 0, 33), vec![3626764237, 0]);
    assert_eq!(gen_bits(0, 0, 63), vec![3626764237, 827307999]);
    assert_eq!(gen_bits(0, 0, 64), vec![3626764237, 1654615998]);
    assert_eq!(gen_bits(21684, 1111, 33), vec![1651504065, 1]);
}

fn gen_i32(seed: u32, iterations: u32, a: i32, b: i32) -> i32 {
    let mut r = Random::new();

    r.seed_u32(seed);

    for _ in 0..iterations {
        r.rand();
    }

    r.rand_i32(a, b)
}

#[test]
fn test_rand_i32() {
    assert_eq!(gen_i32(519876, 8956, 13, 97), 84);
}

fn gen_i32_seed_u64(seed: u64, iterations: u32, a: i32, b: i32) -> i32 {
    let mut r = Random::new();

    r.seed_u64(seed);

    for _ in 0..iterations {
        r.rand();
    }

    r.rand_i32(a, b)
}

#[test]
fn test_rand_i32_seed_u64() {
    assert_eq!(gen_i32_seed_u64(432153415134, 986, -12307, -803), -4223);
}
