#[path = "gungraun/adt.rs"]
mod adt;

#[path = "gungraun/recz.rs"]
mod recz;

use adt::adt as adt_group;
use gungraun::prelude::*;
use gungraun::{Callgrind, FlamegraphConfig};
use recz::{codegen_step, determinization_step, parsing_step, translating_step};

main!(
    config = LibraryBenchmarkConfig::default()
        .tool(Callgrind::default().flamegraph(FlamegraphConfig::default())),
    library_benchmark_groups = [
        adt_group,
        parsing_step,
        translating_step,
        determinization_step,
        codegen_step,
    ]
);
