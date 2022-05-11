// Synopsis: cargo run --bin syn_genome -- $SEED $OUTDIR $GENOMESIZE $DIVRATE $COVERAGE $READLEN $STDDEV
// It would produces several files under ${OUTDIR}.
use hifiasm_phasetest::gen_seq;
use rand::Rng;
use rand::SeedableRng;
use rand_distr::Distribution;
use rand_distr::Normal;
use rand_xoshiro::Xoroshiro128PlusPlus;
use std::io::prelude::*;
use std::io::BufWriter;
fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let seed: u64 = args[1].parse().unwrap();
    let haps = format!("{}/haps.fa", &args[2]);
    let len: usize = args[3].parse().unwrap();
    let div_rate: f64 = args[4].parse::<f64>().unwrap() / 3f64;
    let coverage: usize = args[5].parse().unwrap();
    let readlen: f64 = args[6].parse().unwrap();
    let stddev: f64 = args[7].parse().unwrap();
    let mut rng: Xoroshiro128PlusPlus = SeedableRng::seed_from_u64(seed);
    let hap1 = gen_seq::generate_seq(&mut rng, len);
    use gen_seq::Profile;
    let profile = Profile {
        sub: div_rate,
        ins: div_rate,
        del: div_rate,
    };
    let hap2 = gen_seq::introduce_randomness(&hap1, &mut rng, &profile);
    let mut wtr = std::fs::File::create(&haps).map(BufWriter::new)?;
    writeln!(&mut wtr, ">hap1:{seed}:{:.5}", div_rate * 3f64)?;
    writeln!(&mut wtr, "{}", std::str::from_utf8(&hap1).unwrap())?;
    writeln!(&mut wtr, ">hap2:{seed}:{:.5}", div_rate * 3f64)?;
    writeln!(&mut wtr, "{}", std::str::from_utf8(&hap2).unwrap())?;
    // HiFi error rate, 0.1% error.
    let err = 0.001;
    let read_errors = Profile {
        sub: err,
        ins: err,
        del: err,
    };
    let read_len_dist = Normal::new(readlen, stddev).unwrap();
    let read_num = (coverage * len + readlen.ceil() as usize) / readlen.ceil() as usize;
    let lengths: Vec<_> = read_len_dist
        .sample_iter(&mut rng)
        .take(read_num)
        .enumerate()
        .collect();
    let haps = vec![&hap1, &hap2];
    let reads = format!("{}/reads.fa", &args[2]);
    let mut read_wtr = std::fs::File::create(&reads).map(BufWriter::new)?;
    for (i, len) in lengths {
        let hap_idx = rng.gen_bool(0.5) as usize;
        let hap = &haps[hap_idx];
        let start_pos = rng.gen_range(0..hap.len());
        let is_forward = rng.gen_bool(0.5);
        let read_seq: Vec<_> = hifiasm_phasetest::seq::DNAIter::new(&hap, is_forward)
            .skip(start_pos)
            .take(len.ceil() as usize)
            .collect();
        let read_seq = gen_seq::introduce_randomness(&read_seq, &mut rng, &read_errors);
        writeln!(&mut read_wtr, ">{i},{hap_idx},{is_forward},{start_pos}")?;
        writeln!(&mut read_wtr, "{}", std::str::from_utf8(&read_seq).unwrap())?;
    }
    Ok(())
}
