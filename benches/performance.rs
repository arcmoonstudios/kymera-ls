use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kymera_ls::proto::KymeraConstruct;
use kymera_ls::proto_handlers::ProtoHandler;
use kymera_analysis::analyzer::Analyzer;

fn proto_handler_benchmark(c: &mut Criterion) {
    let handler = ProtoHandler::new();

    c.bench_function("construct_lookup", |b| {
        b.iter(|| {
            black_box(handler.parse_construct("SPACS"));
            black_box(handler.parse_construct("fnc"));
            black_box(handler.parse_construct("MTH"));
            black_box(handler.parse_construct("VERX"));
        })
    });

    c.bench_function("symbol_lookup", |b| {
        b.iter(|| {
            black_box(handler.parse_construct(":>"));
            black_box(handler.parse_construct("~"));
            black_box(handler.parse_construct("|>"));
            black_box(handler.parse_construct("<v?x>"));
        })
    });
}

fn analyzer_benchmark(c: &mut Criterion) {
    let _analyzer = Analyzer::new();

    c.bench_function("analyzer_init", |b| {
        b.iter(|| {
            black_box(Analyzer::new());
        })
    });
}

criterion_group!(benches, proto_handler_benchmark, analyzer_benchmark);
criterion_main!(benches);
