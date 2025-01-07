use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kymera_analysis::analyzer::Analyzer;
use kymera_parser::ast::{ASTNode, FunctionDecl, TypeAnnotation};
use tch::{Device, Tensor, Kind};

fn create_sample_ast() -> Vec<ASTNode> {
    vec![
        ASTNode::FunctionDecl(FunctionDecl {
            name: "test_function".to_string(),
            parameters: vec![],
            return_type: TypeAnnotation::Simple("unit".to_string()),
            body: vec![],
            doc_comment: None,
        })
    ]
}

fn tensor_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("tensor_ops");
    
    // Benchmark tensor creation and basic operations
    group.bench_function("tensor_ops", |b| {
        b.iter(|| {
            let t1 = Tensor::randn(&[1000, 1000], (Kind::Float, Device::Cpu));
            let t2 = Tensor::randn(&[1000, 1000], (Kind::Float, Device::Cpu));
            black_box(t1.matmul(&t2));
        })
    });

    // Benchmark tensor neural network operations
    group.bench_function("neural_ops", |b| {
        b.iter(|| {
            let input = Tensor::randn(&[32, 784], (Kind::Float, Device::Cpu));
            let weights = Tensor::randn(&[784, 128], (Kind::Float, Device::Cpu));
            let bias = Tensor::randn(&[128], (Kind::Float, Device::Cpu));
            black_box(input.matmul(&weights).add(&bias).relu());
        })
    });

    group.finish();
}

fn analyzer_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("analyzer");
    
    // Benchmark analyzer initialization
    group.bench_function("init", |b| {
        b.iter(|| {
            black_box(Analyzer::new());
        })
    });

    // Benchmark simple AST analysis
    let ast = create_sample_ast();
    group.bench_function("analyze_simple_ast", |b| {
        b.iter(|| {
            let mut analyzer = Analyzer::new();
            black_box(analyzer.analyze(&ast[..]));
        })
    });

    group.finish();
}

criterion_group!(benches, analyzer_benchmarks, tensor_benchmarks);
criterion_main!(benches);
