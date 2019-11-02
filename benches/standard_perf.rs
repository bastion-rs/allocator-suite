#![feature(test)]

extern crate test;

#[cfg(test)]
mod standard_perf {
    // Import bencher
    use test::Bencher;

    #[bench]
    pub fn bench_normal_allocator(b: &mut Bencher) {
        use std::alloc::System;

        #[global_allocator]
        static GLOBAL: System = System;

        b.iter(|| {
            let mut vec = Vec::<usize>::with_capacity(10_000_000);
            (0..1_000_000).for_each(|_| {
                vec.push(100);
            });
        })
    }
}
