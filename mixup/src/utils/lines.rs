use rand::rng;
use rand::seq::SliceRandom;

pub fn mixup_lines(raw_sources: Vec<String>, interleave: bool) -> anyhow::Result<()> {
    let raw_sources = if interleave {
        vec![raw_sources.join("")]
    } else {
        raw_sources
    };

    for source in raw_sources {
        let mut lines: Vec<_> = source.lines().collect();
        lines.shuffle(&mut rng());

        for line in lines {
            println!("{line}");
        }
    }

    Ok(())
}
