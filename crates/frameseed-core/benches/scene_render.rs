use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use frameseed_core::{Frame, MandelbrotScene, RenderContext, Scene};

fn bench_mandelbrot(c: &mut Criterion) {
    let mut group = c.benchmark_group("mandelbrot_scene");
    let scene = MandelbrotScene::new(100, -0.05, 0.0, 3.0, 1.0);
    let context = RenderContext::new(0, 100, 24.0, 42);

    for size in [256u32, 512, 1024] {
        group.throughput(Throughput::Elements((size * size) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut frame = Frame::new(size, size);
                scene.render(black_box(&mut frame), black_box(&context));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_mandelbrot);
criterion_main!(benches);
