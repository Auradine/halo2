all: bench

bench:
	RUST_BACKTRACE=full cargo bench --bench sha256 --features unstable

# let config0 = Table16Config {
# 	lookup: SpreadTableConfig {
# 		input: SpreadInputs {
# 			tag: Column {
# 				index: 7,
# 				column_type: Advice,
# 			},
# 			dense: Column {
# 				index: 8,
# 				column_type: Advice,
# 			},
# 			spread: Column {
# 				index: 9,
# 				column_type: Advice,
# 			},
# 		},
# 		table: SpreadTable {
# 			tag: TableColumn {
# 				inner: Column {
# 					index: 0,
# 					column_type: Fixed,
# 				},
# 			},
# 			dense: TableColumn {
# 				inner: Column {
# 					index: 1,
# 					column_type: Fixed,
# 				},
# 			},
# 			spread: TableColumn {
# 				inner: Column {
# 					index: 2,
# 					column_type: Fixed,
# 				},
# 			},
# 		},
# 	},
# 	message_schedule: MessageScheduleConfig {
# 		lookup: SpreadInputs {
# 			tag: Column {
# 				index: 7,
# 				column_type: Advice,
# 			},
# 			dense: Column {
# 				index: 8,
# 				column_type: Advice,
# 			},
# 			spread: Column {
# 				index: 9,
# 				column_type: Advice,
# 			},
# 		},
# 		message_schedule: Column {
# 			index: 0,
# 			column_type: Advice,
# 		},
# 		extras: [
# 			Column {
# 				index: 1,
# 				column_type: Advice,
# 			},
# 			Column {
# 				index: 2,
# 				column_type: Advice,
# 			},
# 			Column {
# 				index: 3,
# 				column_type: Advice,
# 			},
# 			Column {
# 				index: 4,
# 				column_type: Advice,
# 			},
# 			Column {
# 				index: 5,
# 				column_type: Advice,
# 			},
# 			Column {
# 				index: 6,
# 				column_type: Advice,
# 			},
# 		],
# 		s_word: Selector(11, true),
# 		s_decompose_0: Selector(12, true),
# 		s_decompose_1: Selector(13, true),
# 		s_decompose_2: Selector(14, true),
# 		s_decompose_3: Selector(15, true),
# 		s_lower_sigma_0: Selector(16, true),
# 		s_lower_sigma_1: Selector(17, true),
# 		s_lower_sigma_0_v2: Selector(18, true),
# 		s_lower_sigma_1_v2: Selector(19, true),
# 	},
# 	compression: CompressionConfig {
# 		lookup: SpreadInputs {
# 			tag: Column {
# 				index: 7,
# 				column_type: Advice,
# 			},
# 			dense: Column {
# 				index: 8,
# 				column_type: Advice,
# 			},
# 			spread: Column {
# 				index: 9,
# 				column_type: Advice,
# 			},
# 		},
# 		message_schedule: Column {
# 			index: 0,
# 			column_type: Advice,
# 		},
# 		extras: [
# 			Column {
# 				index: 1,
# 				column_type: Advice,
# 			},
# 			Column {
# 				index: 2,
# 				column_type: Advice,
# 			},
# 			Column {
# 				index: 3,
# 				column_type: Advice,
# 			},
# 			Column {
# 				index: 4,
# 				column_type: Advice,
# 			},
# 			Column {
# 				index: 5,
# 				column_type: Advice,
# 			},
# 			Column {
# 				index: 6,
# 				column_type: Advice,
# 			},
# 		],
# 		s_ch: Selector(0, true),
# 		s_ch_neg: Selector(1, true),
# 		s_maj: Selector(2, true),
# 		s_h_prime: Selector(3, true),
# 		s_a_new: Selector(4, true),
# 		s_e_new: Selector(5, true),
# 		s_upper_sigma_0: Selector(6, true),
# 		s_upper_sigma_1: Selector(7, true),
# 		s_decompose_abcd: Selector(8, true),
# 		s_decompose_efgh: Selector(9, true),
# 		s_digest: Selector(10, true),
# 	},
# };


# Usage: cargo run --example cost-model -- [OPTIONS] k
# Positional arguments:
#   k                       2^K bound on the number of rows.
# Optional arguments:
#   -h, --help              Print this message.
#   -a, --advice R[,R..]    An advice column with the given rotations. May be repeated.
#   -i, --instance R[,R..]  An instance column with the given rotations. May be repeated.
#   -f, --fixed R[,R..]     A fixed column with the given rotations. May be repeated.
#   -g, --gate-degree D     Maximum degree of the custom gates.
#   -l, --lookup N,I,T      A lookup over N columns with max input degree I and max table degree T. May be repeated.
#   -p, --permutation N     A permutation over N columns. May be repeated.

cost:
	cargo run --example cost-model -- -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -a 0 -f 0 -f 0 -f 0 -g 0 17

