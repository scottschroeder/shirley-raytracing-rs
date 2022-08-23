use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use raytracer::{
    bvh::{
        aabb::Aabb,
        bbox_tree::{BboxTree, BboxTreeWorkspace},
    },
    camera::{CameraBuilder, CameraPosition},
    core::{
        math::{
            random_in_unit_disk, random_in_unit_sphere, random_on_unit_sphere_distribution,
            random_unit_vector,
        },
        Point, Ray, Vec3,
    },
};

use self::bvh_builder::gen_spheres;

mod bvh_builder {
    use criterion::black_box;
    use rand::{distributions::Uniform, prelude::Distribution, Rng};
    use raytracer::{
        bvh::bbox_tree::{BboxTree, BboxTreeWorkspace},
        core::{math::random_on_unit_sphere_distribution, Point, Ray, Vec3},
        geometry::sphere::Sphere,
    };

    pub struct BvhTester {
        pub side_len: f64,
        pub tree: BboxTree<Sphere>,
    }

    pub fn gen_spheres<R: Rng>(rng: &mut R, side_len: u64) -> Vec<Sphere> {
        let mut spheres = Vec::new();
        let range_start = -(side_len as i64);
        let range_end = side_len as i64;
        let center_offset_distribution = Uniform::new(-1.0, 1.0);
        let sphere_radius_distribution = rand_distr::LogNormal::new(0.5, 0.5).unwrap();
        for x in range_start..range_end {
            for y in range_start..range_end {
                for z in range_start..range_end {
                    let exact_center = Vec3::new(x as f64, y as f64, z as f64);
                    let offset = Vec3::new(
                        center_offset_distribution.sample(rng),
                        center_offset_distribution.sample(rng),
                        center_offset_distribution.sample(rng),
                    );
                    let center = exact_center + offset;
                    let radius = sphere_radius_distribution.sample(rng);
                    spheres.push(Sphere {
                        center: Point(center),
                        radius,
                    });
                }
            }
        }
        spheres
    }

    impl BvhTester {
        pub fn run_once<R: Rng>(&self, rng: &mut R, workspace: &mut BboxTreeWorkspace) {
            let range_distrib = Uniform::new(-self.side_len, self.side_len);
            let orig = Point(Vec3::new(
                range_distrib.sample(rng),
                range_distrib.sample(rng),
                range_distrib.sample(rng),
            ));
            let dir = random_on_unit_sphere_distribution(rng);
            let ray = Ray::new(orig, dir);
            let obj = self.tree.hit_workspace(workspace, &ray, 0.0, std::f64::MAX);
            black_box(obj.is_some());
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    bench_aabb_hit(c);
    bench_bvh(c);
    bench_random(c);
    bench_camera(c);

    // important functions based on flamegraph
    // material scatter
    // sphere hit
    // sphere uv
    // ray_color
}
pub fn bench_bvh(c: &mut Criterion) {
    let scales = (0..4).map(|p| (2u64).pow(p)).collect::<Vec<_>>();

    {
        let mut group = c.benchmark_group("bvh constructor");

        for scale in &scales {
            let mut rng = ChaCha20Rng::seed_from_u64(0xDEADBEEF);
            let spheres = gen_spheres(&mut rng, *scale);

            group.throughput(criterion::Throughput::Elements(spheres.len() as u64));
            group.bench_with_input(BenchmarkId::new("new", spheres.len()), &spheres, |b, i| {
                b.iter_batched(|| i.clone(), BboxTree::new, BatchSize::PerIteration)
            });
        }
    }

    {
        let mut group = c.benchmark_group("bvh hit");

        for scale in &scales {
            let mut rng = ChaCha20Rng::seed_from_u64(0xDEADBEEF);
            let spheres = gen_spheres(&mut rng, *scale);
            let builder = bvh_builder::BvhTester {
                side_len: *scale as f64,
                tree: BboxTree::new(spheres),
            };

            group.throughput(criterion::Throughput::Elements(builder.tree.len() as u64));
            group.bench_with_input(
                BenchmarkId::new("hit", builder.tree.len()),
                &builder,
                |b, i| {
                    let mut rng = ChaCha20Rng::seed_from_u64(0xDEADBEEF);
                    let mut workspace = BboxTreeWorkspace::default();
                    b.iter(|| i.run_once(&mut rng, &mut workspace))
                },
            );
        }
    }
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
