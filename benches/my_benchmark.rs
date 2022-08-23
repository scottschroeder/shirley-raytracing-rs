use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::SeedableRng;
use raytracer::{
    bvh::aabb::Aabb,
    camera::{CameraBuilder, CameraPosition},
    core::{
        math::{
            random_in_unit_disk, random_in_unit_sphere, random_on_unit_sphere_distribution,
            random_unit_vector,
        },
        Point, Ray, Vec3,
    },
};

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_aabb_hit(c);
    bench_random(c);
    bench_camera(c);
}
pub fn bench_aabb_hit(c: &mut Criterion) {
    let bbox = Aabb {
        min: Point(Vec3::new(-1.0, -1.0, 1.0)),
        max: Point(Vec3::new(1.0, 1.0, 2.0)),
    };

    let mut group = c.benchmark_group("aabb hit");
    let ray_hit = Ray::new(Point(Vec3::new(0.0, 0.0, 0.0)), Vec3::new(0.0, 0.0, 1.0));
    let ray_miss = Ray::new(Point(Vec3::new(0.0, 0.0, 0.0)), Vec3::new(0.0, 0.0, -1.0));

    let start = black_box(0.0);
    let end = black_box(std::f64::MAX);

    for ray in &[("ray hit", ray_hit), ("ray miss", ray_miss)] {
        group.bench_with_input(BenchmarkId::new("fn hit", ray.0), &ray.1, |b, i| {
            b.iter(|| bbox.hit(i, start, end))
        });
        group.bench_with_input(BenchmarkId::new("fn hit2", ray.0), &ray.1, |b, i| {
            b.iter(|| bbox.hit2(i, start, end))
        });
    }
}

pub fn bench_random(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    {
        let mut group = c.benchmark_group("random inside unit sphere");
        group.bench_function("by elimination", |b| {
            b.iter(|| random_in_unit_sphere(&mut rng))
        });
    }

    {
        let mut group = c.benchmark_group("random on surface of unit sphere");
        group.bench_function("normalize inside unit sphere", |b| {
            b.iter(|| random_unit_vector(&mut rng))
        });
        group.bench_function("quaternion transform distribution", |b| {
            b.iter(|| random_on_unit_sphere_distribution(&mut rng))
        });
    }

    {
        let mut group = c.benchmark_group("random inside unit disk");
        group.bench_function("by elimination naive", |b| {
            b.iter(|| random_in_unit_disk(&mut rng))
        });
        group.bench_function("by elimination distribution", |b| {
            b.iter(|| random_in_unit_disk(&mut rng))
        });
    }
}
pub fn bench_camera(c: &mut Criterion) {
    let mut builder = CameraBuilder::default();
    builder.width(10).aspect_ratio(1.0);
    let camera = builder.build().unwrap();
    let mut pos = CameraPosition::look_at(
        Point(Vec3::new(278.0, 278.0, -800.0)),
        Point(Vec3::new(278.0, 278.0, 0.0)),
        Vec3::new(0.0, 1.0, 0.0),
    );
    pos.focus_length = 10.0;
    let img_size = 10;

    let mut img = Vec::with_capacity(img_size * img_size);
    for idx in 0..img_size {
        for idy in 0..img_size {
            img.push((idx as f64 + 0.5, idy as f64 + 0.5))
        }
    }

    {
        let mut group = c.benchmark_group("generate pixel ray rng");
        group.bench_function("thread rng", |b| {
            let mut rng = rand::thread_rng();
            b.iter(|| {
                for (x, y) in &img {
                    let r = camera.pixel_ray(&mut rng, black_box(&pos), *x, *y);
                    black_box(r);
                }
            })
        });
        group.bench_function("chacha20 rng", |b| {
            let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(0xDEADBEEF);
            b.iter(|| {
                for (x, y) in &img {
                    let r = camera.pixel_ray(&mut rng, black_box(&pos), *x, *y);
                    black_box(r);
                }
            })
        });
        group.bench_function("small rng", |b| {
            let mut rng = rand::rngs::SmallRng::seed_from_u64(0xDEADBEEF);
            b.iter(|| {
                for (x, y) in &img {
                    let r = camera.pixel_ray(&mut rng, black_box(&pos), *x, *y);
                    black_box(r);
                }
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
