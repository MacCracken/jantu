use criterion::{Criterion, black_box, criterion_group, criterion_main};
use jantu::circadian::{ActivityPattern, CircadianClock, zeitgeber_correction};
use jantu::coevolution::{functional_response_type2, trait_pressure};
use jantu::contagion::{EmotionalState, aggregate_pressure, emotional_influence};
use jantu::genetics::{BehavioralGenome, crossover, genome_fitness};
use jantu::habituation::{HabituationParams, StimulusResponse};
use jantu::instinct::{DriveLevel, Instinct, InstinctType, dominant_instinct};
use jantu::lifecycle::{basal_metabolic_rate, estimated_lifespan_years, heart_rate_bpm};
use jantu::mating::{FitnessTraits, display_cost, mate_acceptance, selection_pressure};
use jantu::memory::{MemoryTrace, MemoryType, social_recognition};
use jantu::migration::{MigrationStrategy, migration_energy_cost, migratory_urge};
use jantu::pack::{food_share, hunt_success_probability};
use jantu::signals::{SignalModality, detection_probability, signal_range};
use jantu::social::{HierarchyPosition, group_cohesion};
use jantu::stress::{StressState, stress_drive_modifier};
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

fn bench_habituation(c: &mut Criterion) {
    let mut group = c.benchmark_group("habituation");
    let params = HabituationParams::default();

    group.bench_function("expose", |b| {
        let mut sr = StimulusResponse::new();
        b.iter(|| sr.expose(black_box(0.5), black_box(&params)));
    });

    group.bench_function("decay", |b| {
        let mut sr = StimulusResponse::new();
        sr.expose(0.5, &params);
        b.iter(|| sr.decay(black_box(1.0), black_box(&params)));
    });

    group.bench_function("response_multiplier", |b| {
        let mut sr = StimulusResponse::new();
        sr.expose(0.5, &params);
        b.iter(|| sr.response_multiplier());
    });

    group.finish();
}

fn bench_circadian(c: &mut Criterion) {
    let mut group = c.benchmark_group("circadian");
    let clock = CircadianClock::new(ActivityPattern::Diurnal);

    group.bench_function("activity_level", |b| {
        b.iter(|| clock.activity_level(black_box(14.5)));
    });

    group.bench_function("drive_modifier", |b| {
        b.iter(|| clock.drive_modifier(black_box(14.5), black_box(false)));
    });

    group.bench_function("zeitgeber_correction", |b| {
        b.iter(|| {
            zeitgeber_correction(
                black_box(2.0),
                black_box(0.0),
                black_box(24.0),
                black_box(0.2),
            )
        });
    });

    group.finish();
}

fn bench_contagion(c: &mut Criterion) {
    let mut group = c.benchmark_group("contagion");

    group.bench_function("emotional_influence", |b| {
        b.iter(|| {
            emotional_influence(
                black_box(0.8),
                black_box(0.7),
                black_box(15.0),
                black_box(100.0),
            )
        });
    });

    group.bench_function("aggregate_pressure_20", |b| {
        let influences: Vec<_> = (0..20)
            .map(|i| {
                let state = if i % 3 == 0 {
                    EmotionalState::Fear
                } else {
                    EmotionalState::Calm
                };
                (0.5 + (i as f32) * 0.02, state)
            })
            .collect();
        b.iter(|| aggregate_pressure(black_box(&influences)));
    });

    group.finish();
}

fn bench_migration(c: &mut Criterion) {
    let mut group = c.benchmark_group("migration");

    group.bench_function("migratory_urge_obligate", |b| {
        b.iter(|| {
            migratory_urge(
                black_box(270),
                black_box(0.5),
                black_box(0.3),
                black_box(MigrationStrategy::Obligate),
            )
        });
    });

    group.bench_function("migration_energy_cost", |b| {
        b.iter(|| migration_energy_cost(black_box(5.0), black_box(true), black_box(0.3)));
    });

    group.finish();
}

fn bench_genetics(c: &mut Criterion) {
    let mut group = c.benchmark_group("genetics");
    let a = BehavioralGenome::default_genome();
    let b = BehavioralGenome::default_genome();

    group.bench_function("crossover", |b_iter| {
        b_iter.iter(|| crossover(black_box(&a), black_box(&b), black_box(&[0.01; 5])));
    });

    group.bench_function("genome_fitness", |b_iter| {
        let weights = [1.0; 5];
        let env = [0.5; 5];
        b_iter.iter(|| genome_fitness(black_box(&a), black_box(&weights), black_box(&env)));
    });

    group.finish();
}

fn bench_signals(c: &mut Criterion) {
    let mut group = c.benchmark_group("signals");

    group.bench_function("signal_range", |b| {
        b.iter(|| signal_range(black_box(SignalModality::Acoustic), black_box(0.8)));
    });

    group.bench_function("detection_probability", |b| {
        b.iter(|| {
            detection_probability(
                black_box(0.7),
                black_box(30.0),
                black_box(100.0),
                black_box(0.2),
            )
        });
    });

    group.finish();
}

fn bench_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory");

    group.bench_function("reinforce", |b| {
        let mut m = MemoryTrace::new(MemoryType::FoodSource, 0.5, 0.5);
        b.iter(|| m.reinforce(black_box(0.4)));
    });

    group.bench_function("forget", |b| {
        let mut m = MemoryTrace::new(MemoryType::Threat, 0.8, -0.5);
        b.iter(|| m.forget(black_box(1.0)));
    });

    group.bench_function("social_recognition", |b| {
        b.iter(|| social_recognition(black_box(8), black_box(50.0)));
    });

    group.finish();
}

fn bench_stress(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress");

    group.bench_function("apply_stressor", |b| {
        let mut s = StressState::new();
        b.iter(|| s.apply_stressor(black_box(0.5)));
    });

    group.bench_function("recover", |b| {
        let mut s = StressState::new();
        s.apply_stressor(0.7);
        b.iter(|| s.recover(black_box(1.0), black_box(0.8)));
    });

    group.bench_function("stress_drive_modifier", |b| {
        b.iter(|| stress_drive_modifier(black_box(0.6), black_box(true)));
    });

    group.finish();
}

fn bench_mating(c: &mut Criterion) {
    let mut group = c.benchmark_group("mating");
    let traits = FitnessTraits {
        condition: 0.8,
        display_quality: 0.7,
        territory_quality: 0.6,
        genetic_quality: 0.7,
        vigor: 0.8,
    };

    group.bench_function("attractiveness", |b| {
        b.iter(|| black_box(&traits).attractiveness());
    });

    group.bench_function("mate_acceptance", |b| {
        b.iter(|| mate_acceptance(black_box(&traits), black_box(0.4), black_box(5)));
    });

    group.bench_function("display_cost", |b| {
        b.iter(|| display_cost(black_box(0.7), black_box(0.8)));
    });

    group.bench_function("selection_pressure", |b| {
        b.iter(|| selection_pressure(black_box(3.0)));
    });

    group.finish();
}

fn bench_coevolution(c: &mut Criterion) {
    let mut group = c.benchmark_group("coevolution");

    group.bench_function("trait_pressure", |b| {
        b.iter(|| trait_pressure(black_box(0.5), black_box(0.8), black_box(0.7)));
    });

    group.bench_function("functional_response_type2", |b| {
        b.iter(|| functional_response_type2(black_box(50.0), black_box(0.5), black_box(0.2)));
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_instinct,
    bench_survival,
    bench_lifecycle,
    bench_pack,
    bench_social,
    bench_swarm,
    bench_territory,
    bench_habituation,
    bench_circadian,
    bench_contagion,
    bench_migration,
    bench_mating,
    bench_coevolution,
    bench_stress,
    bench_memory,
    bench_signals,
    bench_genetics
);
criterion_main!(benches);
