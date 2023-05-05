use std::{marker::PhantomData, time::Instant};

use clap::Parser;
use rand::Rng;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

#[derive(Deserialize)]
struct Outer {
    id: String,
    #[serde(deserialize_with = "deser_max")]
    #[serde(rename(deserialize = "values"))]
    max_value: u64,
}

fn deser_max<'de, T, D>(deserzr: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Ord,
    D: Deserializer<'de>,
{
    struct MaxVisitor<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for MaxVisitor<T>
    where
        T: Deserialize<'de> + Ord,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a nonempty sequence of numbers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let mut max = seq
                .next_element()?
                .ok_or_else(|| de::Error::custom("no values in seq"))?;

            while let Some(v) = seq.next_element()? {
                max = std::cmp::max(max, v);
            }

            Ok(max)
        }
    }

    let visitor = MaxVisitor(PhantomData);
    deserzr.deserialize_seq(visitor)
}

fn mem_effc_parsing(json: &str) -> u64 {
    let out: Outer = serde_json::from_str(json).unwrap();
    out.max_value
}

fn whole_parsing(json: &str) -> u64 {
    let whole: serde_json::Value = serde_json::from_str(json).unwrap();
    let values = whole["values"].as_array().unwrap();

    let mut max = 0;
    for v in values {
        if v.as_u64().unwrap() > max {
            max = v.as_u64().unwrap();
        }
    }

    max
}

fn measure_avg_duration(f: &dyn Fn() -> u64) -> f64 {
    let mut total = 0u128;
    let iteration: usize = 1000;
    for _ in 0..iteration {
        let now = Instant::now();
        f();
        let elapsed = now.elapsed();
        total += elapsed.as_nanos();
    }

    let total = total as f64;

    return total / (iteration as f64);
}

fn make_json(values_len: usize) -> String {
    let mut rng = rand::thread_rng();
    let values: Vec<String> = (1..values_len)
        .map(|_| rng.gen::<u64>())
        .map(|n| format!("{}", n))
        .collect();

    format!(
        r#"{{
        "id": "demo-deserialize-max",
        "values": [
            {}
        ]
    }}"#,
        values.join(",\n")
    )
}

fn bench_for_size(size: usize) {
    let json = make_json(size);
    let avg_nano_secs_1 = measure_avg_duration(&|| mem_effc_parsing(&json));
    let avg_nano_secs_2 = measure_avg_duration(&|| whole_parsing(&json));

    eprintln!("Size,Custom,Whole");
    eprintln!("{},{},{}", size, avg_nano_secs_1, avg_nano_secs_2);
}

#[derive(Parser)]
struct Context {
    #[arg(long)]
    json_size: usize,
}

fn main() {
    let cli = Context::parse();
    bench_for_size(cli.json_size);
}
