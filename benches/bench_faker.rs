#![feature(test)]
mod test {
    extern crate rand;
    extern crate test;
    use faker::Faker;
    use rand::thread_rng;
    use rand::Rng;

    #[bench]
    fn bench_random_from_arr(b: &mut test::Bencher) {
        pub static ADJECTIVE: &'static [&'static str] = &[
            "Adaptive",
            "Advanced",
            "Ameliorated",
            "Assimilated",
            "Automated",
            "Balanced",
            "Visionary",
            "Vision-oriented",
        ];

        b.iter(|| thread_rng().choose(&ADJECTIVE).unwrap())
    }
    #[bench]
    fn bench_random_from_arr_cache_rng(b: &mut test::Bencher) {
        let mut rng = thread_rng();
        pub static ADJECTIVE: &'static [&'static str] = &[
            "Adaptive",
            "Advanced",
            "Ameliorated",
            "Assimilated",
            "Automated",
            "Balanced",
            "Visionary",
            "Vision-oriented",
        ];

        b.iter(|| rng.choose(&ADJECTIVE).unwrap())
    }

    #[bench]
    fn bench_interpol_new_faker(b: &mut test::Bencher) {
        b.iter(|| {
            let facker = Faker::new();
            facker.fake("{{name.last_name}}, {{name.first_name}} {{name.suffix}}")
        })
    }
    #[bench]
    fn bench_interpol_many(b: &mut test::Bencher) {
        b.iter(|| {
            let facker = Faker::new();
            (0..1000)
                .map(|_| facker.fake("{{name.last_name}}, {{name.first_name}} {{name.suffix}}"))
                .collect::<Vec<_>>()
        })
    }
    #[bench]
    fn bench_interpol_many_2(b: &mut test::Bencher) {
        b.iter(|| {
            let facker = Faker::new();
            (0..1000)
                .map(|_| facker.fake2("{{name.last_name}}, {{name.first_name}} {{name.suffix}}"))
                .collect::<Vec<_>>()
        })
    }
    #[bench]
    fn bench_interpol_many_custom(b: &mut test::Bencher) {
        let facker = Faker::new();
        let name = facker.name();
        b.iter(|| {
            (0..1000)
                .map(|_| format!("{}, {} {}", name.last_name(None), name.first_name(None), name.suffix()))
                .collect::<Vec<_>>()
        })
    }
    #[bench]
    fn bench_interpol_reuse_faker(b: &mut test::Bencher) {
        let facker = Faker::new();
        b.iter(|| facker.fake("{{name.last_name}}, {{name.first_name}} {{name.suffix}}"))
    }
}
