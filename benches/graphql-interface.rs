use api_interface::{Apis, DatabaseCredentials};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench(c: &mut Criterion) {
    dotenvy::dotenv().ok();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let db_host = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL");
    let db_host = db_host.replace("http://", "");

    let username = std::env::var("TEST_DATABASE_USERNAME").expect("TEST_DATABASE_USERNAME");
    let password = std::env::var("TEST_DATABASE_PASSWORD").expect("TEST_DATABASE_PASSWORD");
    let db_name = std::env::var("TEST_DATABASE_NAME").expect("TEST_DATABASE_NAME");

    let api_users = std::env::var("TEST_SELLERSHUT_API_USERS").expect("TEST_SELLERSHUT_API_USERS");
    let api_categories =
        std::env::var("TEST_SELLERSHUT_API_CATEGORIES").expect("TEST_SELLERSHUT_API_CATEGORIES");

    let credentials = DatabaseCredentials {
        db_dsn: &db_host,
        db_user: &username,
        db_pass: &password,
        db_ns: "benchmarks",
        db: &db_name,
    };

    let apis = Apis {
        users: &api_users,
        categories: &api_categories,
    };

    let schema = rt
        .block_on(api_interface::ApiSchemaBuilder::new(
            credentials,
            None,
            None,
            apis,
        ))
        .unwrap();

    let schema = schema.build();

    let size = 100;
    let query = |method: &str, count: u16| {
        black_box(format!(
            "
                   query {{
                       {method}(first: {count}) {{
                       edges{{
                         cursor
                         node{{
                           id,
                           name,
                           subCategories,
                           imageUrl
                         }}
                       }},
                       pageInfo {{
                         hasNextPage,
                         hasPreviousPage
                       }}
                     }}
                   }}
                "
        ))
    };

    c.bench_with_input(BenchmarkId::new("categories", size), &size, |b, &s| {
        b.to_async(&rt)
            .iter(|| black_box(schema.execute(query("categories", s))));
    });

    c.bench_with_input(BenchmarkId::new("subCategories", size), &size, |b, &s| {
        b.to_async(&rt)
            .iter(|| black_box(schema.execute(query("subCategories", s))));
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
