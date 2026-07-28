use gungraun::prelude::*;
use recz_adt::{RangeU8, SetU8};
use std::hint::black_box;

#[library_benchmark]
#[benches::one(args = [0..=0, 0..=65, 3..=160, 1..=250])]
fn rangeu8_into_setu8(range: impl Into<RangeU8>) -> SetU8 {
    black_box(SetU8::from(black_box(&range.into())))
}

library_benchmark_group!(name = setu8, benchmarks = rangeu8_into_setu8);

main!(library_benchmark_groups = setu8);
