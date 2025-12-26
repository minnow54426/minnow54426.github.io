use criterion::{black_box, criterion_group, criterion_main, Criterion};
use merkle_rs::MerkleTree;

fn bench_merkle_tree_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkle_tree_construction");

    // Benchmark with 100 leaves
    group.bench_function("100_leaves", |b| {
        let leaves: Vec<Vec<u8>> = (0..100)
            .map(|i| format!("leaf_{}", i).into_bytes())
            .collect();

        b.iter(|| {
            MerkleTree::from_leaves(black_box(leaves.clone()))
        });
    });

    // Benchmark with 1000 leaves
    group.bench_function("1000_leaves", |b| {
        let leaves: Vec<Vec<u8>> = (0..1000)
            .map(|i| format!("leaf_{}", i).into_bytes())
            .collect();

        b.iter(|| {
            MerkleTree::from_leaves(black_box(leaves.clone()))
        });
    });

    // Benchmark with 10000 leaves
    group.bench_function("10000_leaves", |b| {
        let leaves: Vec<Vec<u8>> = (0..10000)
            .map(|i| format!("leaf_{}", i).into_bytes())
            .collect();

        b.iter(|| {
            MerkleTree::from_leaves(black_box(leaves.clone()))
        });
    });

    group.finish();
}

fn bench_merkle_proof_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkle_proof_generation");

    // Benchmark proof generation for different tree sizes
    for size in [100, 1000, 10000] {
        let leaves: Vec<Vec<u8>> = (0..size)
            .map(|i| format!("leaf_{}", i).into_bytes())
            .collect();
        let tree = MerkleTree::from_leaves(leaves);

        group.bench_function(format!("{}_leaves", size), |b| {
            b.iter(|| {
                // Generate proof for a leaf in the middle
                tree.prove(black_box(size / 2))
            });
        });
    }

    group.finish();
}

fn bench_merkle_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("merkle_verification");

    // Benchmark verification for different tree sizes
    for size in [100, 1000, 10000] {
        let leaves: Vec<Vec<u8>> = (0..size)
            .map(|i| format!("leaf_{}", i).into_bytes())
            .collect();
        let tree = MerkleTree::from_leaves(leaves.clone());
        let root = tree.root();
        let proof = tree.prove(size / 2);
        let leaf = &leaves[size / 2];

        group.bench_function(format!("{}_leaves", size), |b| {
            b.iter(|| {
                merkle_rs::verify(black_box(root), black_box(leaf), black_box(proof.clone()))
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_merkle_tree_construction,
    bench_merkle_proof_generation,
    bench_merkle_verification
);
criterion_main!(benches);