use api_core::Listing;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fake::{faker::lorem::en::Words, Fake};
use time::OffsetDateTime;
use uuid::Uuid;

fn bench(c: &mut Criterion) {
    let count = 24;
    let mut categories = Vec::with_capacity(count);

    for _ in 0..count {
        let words: Vec<String> = Words(1..5).fake();
        let words = words.join(" ");

        let tags: Vec<_> = [0; 4].iter().map(|_| Uuid::now_v7()).collect();

        let category = Listing {
            id: Uuid::now_v7(),
            image_url: String::default(),
            user_id: Uuid::now_v7(),
            title: words,
            description: String::default(),
            price: 23.5,
            category_id: Uuid::now_v7(),
            other_images: vec![],
            active: false,
            tags,
            location: String::default(),
            likes: vec![],
            created_at: OffsetDateTime::now_utc(),
            updated_at: None,
            deleted_at: None,
        };

        categories.push(category);
    }

    c.bench_function(&format!("serialise {count}"), |b| {
        b.iter(|| black_box(serde_json::to_string(&categories)))
    });

    let cat_str = serde_json::to_string(&categories).unwrap();

    c.bench_function(&format!("deserialise {count}"), |b| {
        b.iter(|| black_box(serde_json::from_str::<Vec<Listing>>(&cat_str)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);