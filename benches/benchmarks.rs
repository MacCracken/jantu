use criterion::{Criterion, black_box, criterion_group, criterion_main};
use jantu::instinct::{DriveLevel, Instinct, InstinctType, dominant_instinct};
use jantu::lifecycle::{basal_metabolic_rate, estimated_lifespan_years, heart_rate_bpm};
use jantu::pack::{food_share, hunt_success_probability};
use jantu::social::{HierarchyPosition, group_cohesion};
use jantu::survival::select_threat_response;
use jantu::swarm::{path_selection_probability, quorum_reached};
use jantu::territory::territorial_response;

fn bench_instinct(c: &mut Criterion) {
    let mut group = c.benchmark_group("instinct");

    group.bench_function("drive_level_new", |b| {
        b.iter(|| DriveLevel::new(black_box(0.75)));
    });

    group.bench_function("update_priority", |b| {
        let mut instinct = Instinct::new(InstinctType::Fear);
        instinct.drive = DriveLevel::new(0.8);
        b.iter(|| {
            instinct.update_priority();
            black_box(&instinct);
        });
    });

    group.bench_function("dominant_instinct_9", |b| {
        let types = [
            InstinctType::Hunger,
            InstinctType::Thirst,
            InstinctType::Fear,
            InstinctType::Aggression,
            InstinctType::Reproduction,
            InstinctType::Nurturing,
            InstinctType::Curiosity,
            InstinctType::Social,
            InstinctType::Rest,
        ];
        let instincts: Vec<_> = types
            .iter()
            .enumerate()
            .map(|(i, &t)| {
                let mut inst = Instinct::new(t);
                inst.drive = DriveLevel::new(i as f32 / 9.0);
                inst.update_priority();
                inst
            })
            .collect();
        b.iter(|| dominant_instinct(black_box(&instincts)));
    });

    group.finish();
}

fn bench_survival(c: &mut Criterion) {
    c.bench_function("select_threat_response", |b| {
        b.iter(|| {
            select_threat_response(
                black_box(0.7),
                black_box(0.5),
                black_box(1.2),
                black_box(0.6),
            )
        });
    });
}

fn bench_lifecycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("lifecycle");

    group.bench_function("basal_metabolic_rate", |b| {
        b.iter(|| basal_metabolic_rate(black_box(70.0), black_box(70.0)));
    });

    group.bench_function("estimated_lifespan", |b| {
        b.iter(|| estimated_lifespan_years(black_box(70.0), black_box(10.0)));
    });

    group.bench_function("heart_rate_bpm", |b| {
        b.iter(|| heart_rate_bpm(black_box(70.0), black_box(200.0)));
    });

    group.finish();
}

fn bench_pack(c: &mut Criterion) {
    let mut group = c.benchmark_group("pack");

    group.bench_function("hunt_success_probability", |b| {
        b.iter(|| hunt_success_probability(black_box(8), black_box(1.5)));
    });

    group.bench_function("food_share", |b| {
        b.iter(|| food_share(black_box(0.8), black_box(10)));
    });

    group.finish();
}

fn bench_social(c: &mut Criterion) {
    let mut group = c.benchmark_group("social");

    group.bench_function("hierarchy_contest", |b| {
        let a = HierarchyPosition::new(0.9);
        let b_pos = HierarchyPosition::new(0.6);
        b.iter(|| a.contest(black_box(&b_pos), black_box(0.8), black_box(0.7)));
    });

    group.bench_function("group_cohesion_100", |b| {
        let distances: Vec<f32> = (0..100).map(|i| i as f32 * 0.5).collect();
        b.iter(|| group_cohesion(black_box(&distances), black_box(100.0)));
    });

    group.finish();
}

fn bench_swarm(c: &mut Criterion) {
    let mut group = c.benchmark_group("swarm");

    group.bench_function("path_selection_probability", |b| {
        let pheromones = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        b.iter(|| path_selection_probability(black_box(3.0), black_box(&pheromones)));
    });

    group.bench_function("quorum_reached", |b| {
        b.iter(|| quorum_reached(black_box(7), black_box(10), black_box(0.6)));
    });

    group.finish();
}

fn bench_territory(c: &mut Criterion) {
    c.bench_function("territorial_response", |b| {
        b.iter(|| territorial_response(black_box(0.8), black_box(0.5), black_box(0.7)));
    });
}

criterion_group!(
    benches,
    bench_instinct,
    bench_survival,
    bench_lifecycle,
    bench_pack,
    bench_social,
    bench_swarm,
    bench_territory
);
criterion_main!(benches);
