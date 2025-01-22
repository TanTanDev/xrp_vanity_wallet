pub mod num_format;

use std::{
    fs::{read_to_string, File},
    io::Write,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use num_format::format_with_separator;
use xrpl::wallet::Wallet;

fn main() {
    // words we want to find
    let mut lines: Vec<String> = read_to_string("names.txt")
        .expect("names.txt file")
        .lines()
        .map(String::from)
        .collect();
    let lines: Vec<String> = lines.iter_mut().map(|s| s.to_lowercase()).collect();
    let mut handles = vec![];

    // command line visuals
    let cmd_multi_process = MultiProgress::new();
    let total_wallets_bar = cmd_multi_process.add(ProgressBar::new_spinner());
    let spinner_style = ProgressStyle::default_spinner().tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    // screen statistics
    let app_time = Instant::now();
    let total_wallets_searched = Arc::new(AtomicI32::new(0));
    let total_wallets_found = Arc::new(AtomicI32::new(0));

    let cpu_count = num_cpus::get() as i32;
    println!("num cpus: {}", cpu_count);
    println!("FINDING WALLETS");
    // spawn threads
    for i in 0..cpu_count {
        // add 'command line visuals'
        let pb = cmd_multi_process.add(ProgressBar::new(u64::MAX));
        pb.set_style(spinner_style.clone());

        // prepare data
        let lines_clone = lines.clone();
        let wallets_searched = Arc::clone(&total_wallets_searched);
        let wallets_found = Arc::clone(&total_wallets_found);

        // start wallet searching thread
        let handle = std::thread::spawn(move || {
            find_wallets(i, lines_clone, pb, wallets_searched, wallets_found)
        });
        handles.push(handle);
    }
    // report stats to screen, forever, loop keeps thread handles alive
    loop {
        let total = total_wallets_searched.load(Ordering::Relaxed);
        let found = total_wallets_found.load(Ordering::Relaxed);
        let wallets_per_sec = total / (app_time.elapsed().as_secs() as i32).max(1);
        total_wallets_bar.set_message(format!(
            "\ntotal generated: {}\nfound wallets: {}\nwallets generated / sec: {}",
            format_with_separator(total),
            found,
            format_with_separator(wallets_per_sec)
        ));
        std::thread::sleep(Duration::from_secs_f32(1.0));
    }
}

// continously search for wallets for ever
fn find_wallets(
    thread_num: i32,
    lines: Vec<String>,
    cmd_progress_bar: ProgressBar,
    total_generated: Arc<AtomicI32>,
    wallets_found: Arc<AtomicI32>,
) {
    // screen info to record
    let mut wallets_generated = 0i32;
    let mut found_wallets = 0i32;
    let mut last_wallet_name = "".to_string();

    loop {
        // animate progress bar
        if wallets_generated % 2000 == 0 {
            cmd_progress_bar.inc(1);
            cmd_progress_bar.set_message(format!(
                "thread: {}, wallets checked: {}, wallets_found: {}, last name: {}",
                thread_num,
                format_with_separator(wallets_generated),
                found_wallets,
                last_wallet_name
            ));
        }
        wallets_generated += 1;
        total_generated.fetch_add(1, Ordering::Relaxed);

        let wallet = Wallet::create(None).unwrap();

        // we don't care for upper/lower case
        let classic_addr_lower = wallet.classic_address.to_lowercase();

        for word in lines.iter() {
            // does address contain this word, at any point
            let Some(index) = classic_addr_lower.find(word) else {
                continue;
            };
            let at_beginning = index == 1;
            let at_end = index == classic_addr_lower.len() - word.len();
            // we only want the address if the word is at the start or end
            if at_beginning | at_end {
                wallets_found.fetch_add(1, Ordering::Relaxed);
                found_wallets += 1;
                last_wallet_name = word.clone();
                let file_name = format!("output/{}_{}.txt", word, wallets_generated);
                let file_contents = format!(
                    "word: {:?}\nclassic address: {}\n{:?}",
                    word, wallet.classic_address, wallet
                );
                let mut file = File::create(&file_name).unwrap();
                file.write_all(file_contents.as_bytes()).unwrap();
            }
        }
    }
}
