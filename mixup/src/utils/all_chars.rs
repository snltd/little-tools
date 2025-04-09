use rand::rng;
use rand::seq::SliceRandom;

pub fn mixup_chars(raw_sources: Vec<String>, interleave: bool) -> anyhow::Result<()> {
    let sources = if interleave {
        vec![raw_sources.join("")]
    } else {
        raw_sources
    };

    for source in sources {
        let mut chars: Vec<_> = source.chars().collect();
        chars.shuffle(&mut rng());
        let letters: Vec<_> = chars.iter().map(|c| c.to_string()).collect();
        let output = letters.join("");
        println!("{}", output);
    }

    Ok(())
}
