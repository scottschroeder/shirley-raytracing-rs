use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracer::{
    bvh::aabb::Aabb,
    core::{Point, Ray, Vec3},
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let bbox = Aabb {
        min: Point(Vec3::new(-1.0, -1.0, 1.0)),
        max: Point(Vec3::new(1.0, 1.0, 2.0)),
    };
    let ray = Ray::new(Point(Vec3::new(0.0, 0.0, 0.0)), Vec3::new(0.0, 0.0, 1.0));
    c.bench_function("aabb hit", |b| {
        b.iter(|| bbox.hit2(black_box(&ray), black_box(0.0), black_box(std::f64::MAX)))
    });
    let ray2 = Ray::new(Point(Vec3::new(0.0, 0.0, 0.0)), Vec3::new(0.0, 0.0, -1.0));
    c.bench_function("aabb hit (miss)", |b| {
        b.iter(|| bbox.hit2(black_box(&ray2), black_box(0.0), black_box(std::f64::MAX)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
