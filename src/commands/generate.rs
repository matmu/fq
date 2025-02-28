use anyhow::Context;
use clap::ArgMatches;
use rand::{rngs::SmallRng, SeedableRng};
use tracing::info;

use crate::{generator::Builder, Generator, PairWriter};

pub fn generate(matches: &ArgMatches) -> anyhow::Result<()> {
    let r1_dst = matches.value_of("r1-dst").unwrap();
    let r2_dst = matches.value_of("r2-dst").unwrap();

    let record_count = matches
        .value_of_t("record-count")
        .unwrap_or_else(|e| e.exit());
    let read_length = matches
        .value_of_t("read-length")
        .unwrap_or_else(|e| e.exit());

    info!("fq-generate start");

    let builder = if matches.is_present("seed") {
        let seed = matches.value_of_t("seed").unwrap_or_else(|e| e.exit());
        let rng = SmallRng::seed_from_u64(seed);
        Builder::from_rng(rng)
    } else {
        Generator::builder()
    };

    let generator = builder.set_read_length(read_length).build();

    let w1 = crate::fastq::create(r1_dst)
        .with_context(|| format!("Could not create file: {}", r1_dst))?;

    let w2 = crate::fastq::create(r2_dst)
        .with_context(|| format!("Could not create file: {}", r2_dst))?;

    let mut writer = PairWriter::new(w1, w2);

    writer
        .write(generator, record_count)
        .context("Could not write generated records")?;

    info!("generated {} records", record_count);
    info!("fq-generate end");

    Ok(())
}
