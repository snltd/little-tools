use rand::rng;
use rand::seq::SliceRandom;

pub fn mixup_lines(raw_sources: Vec<String>, interleave: bool) -> anyhow::Result<()> {
    let sources = if interleave {
        let combined_ordered_source = raw_sources.join("");
        let mut lines: Vec<_> = combined_ordered_source.lines().to_owned().collect();
        lines.shuffle(&mut rng());
        let combined_shuffled_source = lines.join("\n");
        vec![combined_shuffled_source]
    } else {
        raw_sources
    };

    for source in sources {
        let original_lines: Vec<_> = source.lines().collect();

        let shuffled_lines: Vec<String> = original_lines.into_iter().map(shuffle_line).collect();

        for line in shuffled_lines {
            println!("{line}");
        }
    }

    Ok(())
}

fn shuffle_line(line: &str) -> String {
    let mut words: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    words.shuffle(&mut rng());
    words.join(" ")
}
