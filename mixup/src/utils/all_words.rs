use rand::rng;
use rand::seq::SliceRandom;

pub fn mixup_words(raw_sources: Vec<String>, interleave: bool) -> anyhow::Result<()> {
    let sources = if interleave {
        vec![raw_sources.join("")]
    } else {
        raw_sources
    };

    for source in sources {
        let mut words: Vec<_> = source.split_whitespace().collect();
        words.shuffle(&mut rng());
        let output = words.join(" ");
        println!("{output}");
    }

    Ok(())
}
