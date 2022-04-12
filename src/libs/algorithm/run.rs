//
// ────────────────────────────────────────────── I ──────────
//   :::::: R U N : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────
//

// Allowing Camel Case
#![allow(non_snake_case)]
use crate::*;

macro_rules! diffcomparison {
	($MChunk: expr, $IChunk: expr, $VChunk: expr) => {
		$MChunk.iter().sum::<f32>() - $IChunk.iter().sum::<u16>() as f32 / $VChunk.iter().sum::<u16>() as f32
	};
}

// pub fn run(rawinput: String, learnt: (Vec<Vec<u16>>, Vec<Vec<f32>>), memory: Option<usize>, threshold: Option<f32>) {
// 	match (memory, threshold) {
// 		(None, None) => __run__(rawinput, learnt, DEFAULT_MEMORY, DEFAULT_THRESHOLD),
// 		(Some(mem), None) => __run__(rawinput, learnt, mem, DEFAULT_THRESHOLD),
// 		(None, Some(thr)) => __run__(rawinput, learnt, DEFAULT_MEMORY, thr),
// 		(Some(mem), Some(thr)) => __run__(rawinput, learnt, mem, thr)
// 	};
// }

pub fn __run__(rawinput: String, learnt: (&Map::<Vec<u16>>, &Vec<Vec<f32>>, &Vec<Vec<String>>), memory: usize, threshold: f32) -> String {
	// * Input translation
	let mut input: Vec<u16> = Vec::new();

	for word in rawinput.split_whitespace() {
		for c in word.chars() {
			input.push((c as u16).pow(11/10));
		};
	};

	// Let's alias some things
	let TMap = learnt.0;
	let Mega = learnt.1;
	let RValues = learnt.2;

	// Algorithm fixed (Being
	
		// O(#C(n) * #C(m))
	
	// instead of
	
	// O( sum_{i <= #V} #C(K_i)^#C(V) )
	
	// )

	let IRM :usize;
	let mut KRM :usize;
	let mut VRM :usize;
	let mut MRM :usize;
	let mut FRM :usize;

	let mut UBM :(usize, usize, usize, usize) = (0, 0, 0, 0);

	let mut KeyChunks: Chunks<u16>;
	let mut ValueChunks: Chunks<u16> = Chunks::new();

	let mut BestMatch: Option
	<(
	
		usize,
		usize,
		usize,
		usize
		
	)>;

	let mut BMCalculation;
	let mut Calculation: f32;

	let mut result: String = String::new();

	checkmem!(memory, input, IRM);

	for IChunk in input.into_chunks(IRM).base {
		BestMatch = None;
		for (i, (key, value)) in TMap.iter().enumerate() {
			checkmem!(memory, key, KRM, value, VRM, Mega[i], MRM);
			KeyChunks = key.into_chunks(KRM);
			ValueChunks = value.into_chunks(VRM);
			for (vi, &VChunk) in ValueChunks.base.iter().enumerate() {
				for ki in 0..KeyChunks.base.len() {
					for (m, &MChunk) in Mega[i].into_chunks(MRM).base.iter().enumerate() {

						Calculation = diffcomparison!(MChunk, IChunk, VChunk);

						if Calculation <= threshold {
							match BestMatch {
								None => {
									BestMatch = Some((i, ki, vi, m));
								},
								Some((i, ki, vi, m)) => {
									UBM = BestMatch.unwrap();
									BMCalculation = diffcomparison!(Mega[UBM.3], KeyChunks.base[UBM.1], ValueChunks.base[UBM.2]);
									if Calculation < BMCalculation {
										BestMatch = Some((i, ki, vi, m));
									};
								}
							};
						};
					};
				};
			};
			// After we analise that pair, we can now pass to the next.
		};
		// After we analise all the pairs, we can now decide on one.

		// Let's check if the chunk elected for this is at the end of the elected Key
		// We do this because maybe the answer to X is shorter than the input.

		if BestMatch == None {
			continue;

		} else {

			checkmem!(memory, RValues[BestMatch.unwrap().0], FRM);
			result.push_str(&RValues[UBM.0].into_chunks(FRM).base[UBM.3].join("\\/"));
		};


		// If the chunk elected for this is at the end of the elected Key we break the loop.

		if UBM.2 == ValueChunks.base.len() - 1 {
			break;
		};
	};
	return result
}