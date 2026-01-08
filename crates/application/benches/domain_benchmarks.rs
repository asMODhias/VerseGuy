use criterion::{black_box, criterion_group, criterion_main, Criterion};
use verseguy_application::{AddMemberDto, ApplicationService, CreateOrganizationDto};

fn benchmark_create_organization(c: &mut Criterion) {
    let svc = ApplicationService::new();
    c.bench_function("create_organization", |b| {
        b.iter(|| {
            let dto = CreateOrganizationDto {
                name: black_box("Test Org".to_string()),
                tag: black_box("TEST".to_string()),
                description: black_box("Desc".to_string()),
            };
            let _ = svc.create_organization(dto, black_box("user_1".to_string()));
        })
    });
}

fn benchmark_add_member(c: &mut Criterion) {
    let svc = ApplicationService::new();
    let org = svc
        .create_organization(
            CreateOrganizationDto {
                name: "Org".to_string(),
                tag: "TST".to_string(),
                description: "D".to_string(),
            },
            "u".to_string(),
        )
        .unwrap();
    c.bench_function("add_member", |b| {
        let mut i = 0usize;
        b.iter(|| {
            i += 1;
            let dto = AddMemberDto {
                organization_id: org.id.clone(),
                user_id: format!("user_{}", i),
            };
            let _ = svc.add_member(dto, black_box("admin".to_string()));
        })
    });
}

criterion_group!(benches, benchmark_create_organization, benchmark_add_member);
criterion_main!(benches);
