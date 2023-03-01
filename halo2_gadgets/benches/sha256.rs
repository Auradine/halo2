use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner},
    pasta::{pallas, EqAffine},
    plonk::{
        create_proof, keygen_pk, keygen_vk, verify_proof, Circuit, ConstraintSystem, Error,
        SingleVerifier,
    },
    poly::commitment::Params,
    transcript::{Blake2bRead, Blake2bWrite, Challenge255},
};
use rand::rngs::OsRng;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use criterion::{criterion_group, criterion_main, Criterion};

use halo2_gadgets::sha256::{BlockWord, Sha256, Table16Chip, Table16Config, BLOCK_SIZE};

use halo2_gadgets::sha256_input::get_input;

#[allow(dead_code)]
fn bench(name: &str, k: u32, c: &mut Criterion) {
    #[derive(Debug, Default, Clone, Copy)]
    struct MyCircuit {}

    impl Circuit<pallas::Base> for MyCircuit {
        type Config = Table16Config;
        type FloorPlanner = SimpleFloorPlanner;

        fn without_witnesses(&self) -> Self {
            Self::default()
        }

        fn configure(meta: &mut ConstraintSystem<pallas::Base>) -> Self::Config {
            let config = Table16Chip::configure(meta);
            // println!("config = {:?}", config);

            // let config0 = Table16Config {
            //     lookup: SpreadTableConfig {
            //         input: SpreadInputs {
            //             tag: Column {
            //                 index: 7,
            //                 column_type: Advice,
            //             },
            //             dense: Column {
            //                 index: 8,
            //                 column_type: Advice,
            //             },
            //             spread: Column {
            //                 index: 9,
            //                 column_type: Advice,
            //             },
            //         },
            //         table: SpreadTable {
            //             tag: TableColumn {
            //                 inner: Column {
            //                     index: 0,
            //                     column_type: Fixed,
            //                 },
            //             },
            //             dense: TableColumn {
            //                 inner: Column {
            //                     index: 1,
            //                     column_type: Fixed,
            //                 },
            //             },
            //             spread: TableColumn {
            //                 inner: Column {
            //                     index: 2,
            //                     column_type: Fixed,
            //                 },
            //             },
            //         },
            //     },
            //     message_schedule: MessageScheduleConfig {
            //         lookup: SpreadInputs {
            //             tag: Column {
            //                 index: 7,
            //                 column_type: Advice,
            //             },
            //             dense: Column {
            //                 index: 8,
            //                 column_type: Advice,
            //             },
            //             spread: Column {
            //                 index: 9,
            //                 column_type: Advice,
            //             },
            //         },
            //         message_schedule: Column {
            //             index: 0,
            //             column_type: Advice,
            //         },
            //         extras: [
            //             Column {
            //                 index: 1,
            //                 column_type: Advice,
            //             },
            //             Column {
            //                 index: 2,
            //                 column_type: Advice,
            //             },
            //             Column {
            //                 index: 3,
            //                 column_type: Advice,
            //             },
            //             Column {
            //                 index: 4,
            //                 column_type: Advice,
            //             },
            //             Column {
            //                 index: 5,
            //                 column_type: Advice,
            //             },
            //             Column {
            //                 index: 6,
            //                 column_type: Advice,
            //             },
            //         ],
            //         s_word: Selector(11, true),
            //         s_decompose_0: Selector(12, true),
            //         s_decompose_1: Selector(13, true),
            //         s_decompose_2: Selector(14, true),
            //         s_decompose_3: Selector(15, true),
            //         s_lower_sigma_0: Selector(16, true),
            //         s_lower_sigma_1: Selector(17, true),
            //         s_lower_sigma_0_v2: Selector(18, true),
            //         s_lower_sigma_1_v2: Selector(19, true),
            //     },
            //     compression: CompressionConfig {
            //         lookup: SpreadInputs {
            //             tag: Column {
            //                 index: 7,
            //                 column_type: Advice,
            //             },
            //             dense: Column {
            //                 index: 8,
            //                 column_type: Advice,
            //             },
            //             spread: Column {
            //                 index: 9,
            //                 column_type: Advice,
            //             },
            //         },
            //         message_schedule: Column {
            //             index: 0,
            //             column_type: Advice,
            //         },
            //         extras: [
            //             Column {
            //                 index: 1,
            //                 column_type: Advice,
            //             },
            //             Column {
            //                 index: 2,
            //                 column_type: Advice,
            //             },
            //             Column {
            //                 index: 3,
            //                 column_type: Advice,
            //             },
            //             Column {
            //                 index: 4,
            //                 column_type: Advice,
            //             },
            //             Column {
            //                 index: 5,
            //                 column_type: Advice,
            //             },
            //             Column {
            //                 index: 6,
            //                 column_type: Advice,
            //             },
            //         ],
            //         s_ch: Selector(0, true),
            //         s_ch_neg: Selector(1, true),
            //         s_maj: Selector(2, true),
            //         s_h_prime: Selector(3, true),
            //         s_a_new: Selector(4, true),
            //         s_e_new: Selector(5, true),
            //         s_upper_sigma_0: Selector(6, true),
            //         s_upper_sigma_1: Selector(7, true),
            //         s_decompose_abcd: Selector(8, true),
            //         s_decompose_efgh: Selector(9, true),
            //         s_digest: Selector(10, true),
            //     },
            // };

            return config;
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<pallas::Base>,
        ) -> Result<(), Error> {
            Table16Chip::load(config.clone(), &mut layouter)?;
            let table16_chip = Table16Chip::construct(config);

            // Test vector: "abc"
            // let test_input = [
            //     BlockWord(Some(0b01100001011000100110001110000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     BlockWord(Some(0b00000000000000000000000000000000)),
            //     // BlockWord(Some(0b00000000000000000000000000011000)),
            // ];

            let test_input = get_input();

            // println!("\ntest_input: {:?}", test_input);
            let sum = Sha256::digest(table16_chip, layouter.namespace(|| "'abc'"), &test_input);
            println!("\nsum: {:?}", sum);

            Ok(())
        }
    }

    let mut group = c.benchmark_group("sha256");
    group.sample_size(10);

    // Initialize the polynomial commitment parameters
    let params_path = Path::new("./benches/sha256_assets/sha256_params");
    if File::open(&params_path).is_err() {
        let params: Params<EqAffine> = Params::new(k);
        let mut buf = Vec::new();

        params.write(&mut buf).expect("Failed to write params");
        let mut file = File::create(&params_path).expect("Failed to create sha256_params");

        file.write_all(&buf[..])
            .expect("Failed to write params to file");
    }

    let params_fs = File::open(&params_path).expect("couldn't load sha256_params");
    let params: Params<EqAffine> =
        Params::read::<_>(&mut BufReader::new(params_fs)).expect("Failed to read params");

    let empty_circuit: MyCircuit = MyCircuit {};

    // Initialize the proving key
    let vk = keygen_vk(&params, &empty_circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk, &empty_circuit).expect("keygen_pk should not fail");

    let circuit: MyCircuit = MyCircuit {};

    let prover_name = name.to_string() + "-prover";
    let verifier_name = name.to_string() + "-verifier";

    // Benchmark proof creation
    let mut loop_idx = 0;
    group.bench_function(&prover_name, |b| {
        b.iter(|| {
            // println!("\n{}:{}", &prover_name, loop_idx);
            loop_idx += 1;

            let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
            create_proof(&params, &pk, &[circuit], &[], OsRng, &mut transcript)
                .expect("proof generation should not fail");

            let _proof: Vec<u8> = transcript.finalize();
        });
    });

    // Create a proof
    let proof_path = Path::new("./benches/sha256_assets/sha256_proof");
    if true {
        // File::open(&proof_path).is_err() {
        let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
        create_proof(&params, &pk, &[circuit], &[], OsRng, &mut transcript)
            .expect("proof generation should not fail");
        let proof: Vec<u8> = transcript.finalize();
        let mut file = File::create(&proof_path).expect("Failed to create sha256_proof");
        file.write_all(&proof[..]).expect("Failed to write proof");
    }

    // println!("circuit = {:?}", circuit);

    let mut proof_fs = File::open(&proof_path).expect("couldn't load sha256_proof");
    let mut proof = Vec::<u8>::new();
    proof_fs
        .read_to_end(&mut proof)
        .expect("Couldn't read proof");

    loop_idx = 0;
    group.bench_function(&verifier_name, |b| {
        b.iter(|| {
            // println!("\n{}:{}", &verifier_name, loop_idx);
            loop_idx += 1;

            let strategy = SingleVerifier::new(&params);
            let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
            assert!(verify_proof(&params, pk.get_vk(), strategy, &[], &mut transcript).is_ok());
        });
    });
}

#[allow(dead_code)]
fn criterion_benchmark(c: &mut Criterion) {
    bench("sha256", 17, c);
    // bench("sha256", 20, c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
