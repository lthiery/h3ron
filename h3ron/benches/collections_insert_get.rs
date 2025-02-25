use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use geo_types::Coordinate;

use h3ron::collections::indexhierarchy::IndexHierarchyMap;
use h3ron::collections::{H3CellMap, H3Treemap, RandomState, ThreadPartitionedMap};
use h3ron::H3Cell;

fn criterion_benchmark(c: &mut Criterion) {
    let cells = H3Cell::from_coordinate(&Coordinate::from((12.3, 45.4)), 10)
        .unwrap()
        .k_ring(2000)
        .iter()
        .collect::<Vec<_>>();
    let value = 78u8;

    let mut group = c.benchmark_group("collections");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(3));
    group.bench_function(
        format!("H3CellMap::from_iter (n={})", cells.len()),
        |bencher| {
            bencher.iter(|| {
                H3CellMap::from_iter(cells.iter().map(|cell| (*cell, value)));
            });
        },
    );
    group.bench_function(
        format!("ThreadPartitionedMap::from_iter (n={})", cells.len()),
        |bencher| {
            bencher.iter(|| {
                ThreadPartitionedMap::<_, _, 4>::from_iter(cells.iter().map(|cell| (*cell, value)));
            });
        },
    );
    group.bench_function(
        format!("H3Treemap::from_iter_with_sort (n={})", cells.len()),
        |bencher| {
            bencher.iter(|| {
                H3Treemap::from_iter_with_sort(cells.iter().copied());
            });
        },
    );
    group.bench_function(
        format!("IndexHierarchyMap::from_iter (n={})", cells.len()),
        |bencher| {
            bencher.iter(|| {
                IndexHierarchyMap::from_iter(cells.iter().map(|cell| (*cell, value)));
            });
        },
    );
    group.bench_function(format!("H3CellMap.get (len={})", cells.len()), |bencher| {
        let map = H3CellMap::from_iter(cells.iter().map(|cell| (*cell, value)));
        bencher.iter(|| map.get(&cells[0]).unwrap());
    });
    group.bench_function(
        format!("H3Treemap.contains (len={})", cells.len()),
        |bencher| {
            let map = H3Treemap::from_iter(cells.iter());
            bencher.iter(|| map.contains(&cells[0]));
        },
    );
    group.bench_function(
        format!("IndexHierarchyMap.get (len={})", cells.len()),
        |bencher| {
            let map = IndexHierarchyMap::from_iter(cells.iter().map(|cell| (*cell, value)));
            bencher.iter(|| map.get(&cells[0]).unwrap());
        },
    );
    group.bench_function(format!("BTreeMap.get (len={})", cells.len()), |bencher| {
        let map = BTreeMap::from_iter(cells.iter().map(|cell| (*cell, value)));
        bencher.iter(|| map.get(&cells[0]).unwrap());
    });
    group.bench_function(
        format!("std::collections::HashMap.get (len={}; ahash)", cells.len()),
        |bencher| {
            let mut map = std::collections::HashMap::with_hasher(RandomState::new());
            for cell in cells.iter() {
                map.insert(*cell, value);
            }
            bencher.iter(|| map.get(&cells[0]).unwrap());
        },
    );
    group.bench_function(
        format!("ThreadPartitionedMap.get (len={})", cells.len()),
        |bencher| {
            let map =
                ThreadPartitionedMap::<_, _, 4>::from_iter(cells.iter().map(|cell| (*cell, value)));
            bencher.iter(|| map.get(&cells[0]).unwrap());
        },
    );
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
