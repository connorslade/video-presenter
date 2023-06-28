use std::ops::{Deref, DerefMut};

use anyhow::Result;

use crate::time::{time, Time};

/// A collection of time cues.
/// Guaranteed to be sorted with the first cue being 0.
pub struct Cues {
    inner: Vec<Time>,
}

impl Cues {
    /// Load cues from a file.
    /// This file can be created from Adobe Premiere Pro when [exporting markers](https://community.adobe.com/t5/premiere-pro-discussions/how-to-export-clip-markers/td-p/10337163).
    /// If using After Effects, you can use [Marker Batch Editor Script](https://aescripts.com/marker-batch-editor) with this output formatter: `,,[time],[time],[markerDuration],Cue Point\n`.
    ///
    /// If you use something else, like Resolve, Im sure you can figure it out.
    /// Just make sure that:
    /// - The file is tab or comma separated
    /// - Index 2 and 3 are the time of the marker (ex: HH:MM:SS:FF)
    /// - Index 5 is the marker type (Must contain 'Cue Point')
    pub fn from_str(contents: &str) -> Result<Self> {
        let mut inner = Vec::new();
        let header_present = contents.chars().next().unwrap().is_alphabetic();

        for (i, line) in contents.lines().skip(header_present as usize).enumerate() {
            let parts = line.split(['\t', ',']).collect::<Vec<_>>();

            if !parts[5].contains("Cue Point") {
                eprintln!(
                    "[WARN] Skipping marker {} because it is not a 'Cue Point'",
                    i + 1
                );
                continue;
            }

            let start = parts[2].parse::<Time>()?;
            let end = parts[3].parse::<Time>()?;
            if start != end {
                eprintln!(
                    "[WARN] Skipping marker {} because it has a nob-zero duration",
                    i + 1
                );
                continue;
            }

            inner.push(start);
        }

        inner.sort();
        Ok(Self { inner })
    }

    /// Gets the number of cues.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Uses the current time (in seconds) and the fps to get the current cue.
    /// If not exactly on a cue, it will return the last cue.
    /// If the time is before the first cue, it will return 0 and if the time is after the last cue, it will return [`Time::END`].
    pub fn current(&self, time: f64, fps: f64) -> usize {
        for (i, e) in self.inner.iter().enumerate().rev() {
            if time >= e.as_secs(fps) {
                return i + 1;
            }
        }

        if time >= self.inner.last().unwrap_or(&Time::END).as_secs(fps) {
            return self.len() + 1;
        }

        0
    }

    /// Gets the cue at the given index.
    /// If the index is 0, it will return a time of 00:00:00:00.
    pub fn get(&self, idx: usize) -> Time {
        if idx == 0 {
            return time!(00:00:00:00);
        }

        *self.inner.get(idx - 1).unwrap_or(&Time::END)
    }
}

impl Deref for Cues {
    type Target = Vec<Time>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Cues {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::Cues;
    use crate::time::{time, Time};

    use indoc::indoc;

    const TIMES: [Time; 11] = [
        time!(00:00:04:57),
        time!(00:00:21:31),
        time!(00:00:30:50),
        time!(00:00:38:28),
        time!(00:00:43:16),
        time!(00:01:12:53),
        time!(00:01:35:45),
        time!(00:01:58:28),
        time!(00:02:06:29),
        time!(00:02:18:55),
        time!(00:02:24:18),
    ];

    #[test]
    fn test_parse_premiere_cues() {
        const CONTENTS: &str = indoc! { r#"
            Marker Name	Description	In	Out	Duration	Marker Type	
            		00:00:04:57	00:00:04:57	00:00:00:00	Flash Cue Point	
            		00:00:21:31	00:00:21:31	00:00:00:00	Flash Cue Point	
            		00:00:30:50	00:00:30:50	00:00:00:00	Flash Cue Point	
            		00:00:38:28	00:00:38:28	00:00:00:00	Flash Cue Point	
            		00:00:43:16	00:00:43:16	00:00:00:00	Flash Cue Point	
            		00:01:12:53	00:01:12:53	00:00:00:00	Flash Cue Point	
            		00:01:35:45	00:01:35:45	00:00:00:00	Flash Cue Point	
            		00:01:58:28	00:01:58:28	00:00:00:00	Flash Cue Point	
            		00:02:06:29	00:02:06:29	00:00:00:00	Flash Cue Point	
            		00:02:18:55	00:02:18:55	00:00:00:00	Flash Cue Point	
            		00:02:24:18	00:02:24:18	00:00:00:00	Flash Cue Point	
            "#
        };

        let cues = Cues::from_str(CONTENTS).unwrap();
        assert_eq!(cues.len(), TIMES.len());

        for (a, b) in cues.inner.iter().zip(TIMES.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_parse_after_effects_cues() {
        const CONTENTS: &str = indoc! { r#"
            ,,00:00:04:57,00:00:04:57,0,Cue Point
            ,,00:00:21:31,00:00:21:31,0,Cue Point
            ,,00:00:30:50,00:00:30:50,0,Cue Point
            ,,00:00:38:28,00:00:38:28,0,Cue Point
            ,,00:00:43:16,00:00:43:16,0,Cue Point
            ,,00:01:12:53,00:01:12:53,0,Cue Point
            ,,00:01:35:45,00:01:35:45,0,Cue Point
            ,,00:01:58:28,00:01:58:28,0,Cue Point
            ,,00:02:06:29,00:02:06:29,0,Cue Point
            ,,00:02:18:55,00:02:18:55,0,Cue Point
            ,,00:02:24:18,00:02:24:18,0,Cue Point
            "#
        };

        let cues = Cues::from_str(CONTENTS).unwrap();
        assert_eq!(cues.len(), TIMES.len());

        for (a, b) in cues.inner.iter().zip(TIMES.iter()) {
            assert_eq!(a, b);
        }
    }
}
