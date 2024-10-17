use assert_cmd::Command;
use criterion::{criterion_group, criterion_main, Criterion};
use std::io::Write;
use tempfile::NamedTempFile;

fn bench_tc_command(c: &mut Criterion) {
    let mut group = c.benchmark_group("tc_command");

    // Benchmark with small input
    let small_input = "This is a small input\nfor benchmarking purposes.";
    bench_with_input(&mut group, "small_input", small_input);

    // Benchmark with medium input
    let medium_input = "This is a medium input\n".repeat(100);
    bench_with_input(&mut group, "medium_input", &medium_input);

    // Benchmark with large input
    let large_input = "This is a large input\n".repeat(1000);
    bench_with_input(&mut group, "large_input", &large_input);

    group.finish();
}

fn bench_with_input(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    name: &str,
    input: &str,
) {
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "{}", input).unwrap();

    group.bench_function(name, |b| {
        b.iter(|| {
            Command::cargo_bin("tc")
                .unwrap()
                .arg(temp_file.path())
                .assert()
                .success();
        });
    });
}

fn bench_multiple_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("tc_multiple_files");
    let file_contents = "This is a test file\nwith multiple lines\nfor benchmarking purposes.\n안녕하세요, 이것은 한국어 문장입니다.\nこんにちは、これは日本語の文章です。\n";

    for &num_files in &[1, 10, 50, 500] {
        let temp_files: Vec<NamedTempFile> = (0..num_files)
            .map(|_| {
                let mut file = NamedTempFile::new().unwrap();
                write!(file, "{}", file_contents).unwrap();
                file
            })
            .collect();

        let file_paths: Vec<_> = temp_files
            .iter()
            .map(|f| f.path().to_str().unwrap())
            .collect();

        group.bench_function(format!("{}_files", num_files), |b| {
            b.iter(|| {
                let mut cmd = Command::cargo_bin("tc").unwrap();
                cmd.arg("-l").args(&file_paths).assert().success();
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_tc_command, bench_multiple_files);
criterion_main!(benches);
