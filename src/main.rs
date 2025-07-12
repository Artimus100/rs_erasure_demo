use reed_solomon_erasure::ReedSolomon;
use rand::Rng;

fn split_data(data: &[u8], k: usize) -> Vec<Vec<u8>> {
    let shard_size = (data.len() + k - 1) / k; // Ceiling division
    let mut shards = Vec::with_capacity(k);
    let mut start = 0;
    for _ in 0..k {
        let end = start + shard_size;
        let mut shard = if end <= data.len() {
            data[start..end].to_vec()
        } else {
            let mut s = data[start..].to_vec();
            s.resize(shard_size, 0); // Pad with zeros
            s
        };
        shards.push(shard);
        start = end;
    }
    shards
}

fn combine_shards(shards: &[Option<Vec<u8>>], original_len: usize) -> Vec<u8> {
    let mut combined = shards.iter().filter_map(|s| s.as_ref()).flatten().cloned().collect::<Vec<u8>>();
    combined.truncate(original_len);
    combined
}

fn main() {
    // Sample input data
    let data = "Hello, Solana!".as_bytes();
    let k = 4; // Number of data shards
    let m = 2; // Number of parity shards
    let n = k + m; // Total number of shards

    // Step 1: Split data into k shards with padding
    let data_shards = split_data(data, k);
    let shard_size = data_shards[0].len();

    // Step 2: Create Reed-Solomon encoder
    let rsc = ReedSolomon::<reed_solomon_erasure::galois_8::Field>::new(k, m)
        .expect("Failed to create Reed-Solomon encoder");

    // Step 3: Encode data to generate parity shards
    let mut all_shards: Vec<Vec<u8>> = data_shards;
    all_shards.extend(vec![vec![0u8; shard_size]; m]);
    rsc.encode(&mut all_shards).expect("Encoding failed");

    // Step 4: Simulate transmission loss
    let mut rng = rand::thread_rng();
    let loss_count = rng.gen_range(1..=m); // Lose up to m shards
    println!("Simulating loss of {} shards", loss_count);
    let mut shards_option: Vec<Option<Vec<u8>>> = all_shards.into_iter().map(Some).collect();
    for _ in 0..loss_count {
        let idx = rng.gen_range(0..n);
        shards_option[idx] = None;
    }

    // Step 5: Reconstruct the lost shards
    rsc.reconstruct(&mut shards_option).expect("Reconstruction failed");

    // Step 6: Combine the first k shards to recover the original data
    let reconstructed: Vec<u8> = combine_shards(&shards_option[0..k], data.len());

    // Output results
    println!("Original: {:?}", String::from_utf8(data.to_vec()).unwrap());
    println!("Reconstructed: {:?}", String::from_utf8(reconstructed).unwrap());
}