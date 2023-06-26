use std::{
    fmt::{self, Display},
    str::FromStr,
    time::Duration,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Time {
    hours: u8,
    minutes: u8,
    seconds: u8,
    frames: u8,
}

impl Time {
    pub fn as_frames(&self, fps: f32) -> u32 {
        let seconds = self.seconds as u32 + self.minutes as u32 * 60 + self.hours as u32 * 3600;
        self.frames as u32 + (seconds as f32 * fps) as u32
    }

    pub fn from_duration(duration: Duration, fps: f32) -> Self {
        let seconds = duration.as_secs_f32();
        let frames = (seconds * fps) as u32;
        let seconds = seconds as u32;

        Self {
            hours: (seconds / 3600) as u8,
            minutes: (seconds / 60) as u8,
            seconds: (seconds % 60) as u8,
            frames: (frames % fps as u32) as u8,
        }
    }
}

impl FromStr for Time {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut time = [0; 4];
        let parts = s.split(':');
        for (i, part) in parts.rev().enumerate() {
            time[3 - i] = part.parse()?;
        }

        Ok(Time {
            hours: time[0],
            minutes: time[1],
            seconds: time[2],
            frames: time[3],
        })
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{:<02}:{:<02}:{:<02}:{:<02}",
            self.hours, self.minutes, self.seconds, self.frames
        ))
    }
}

// Macro to parse the "hh:mm::ss:ff" format into a Time at compile time
pub macro time($hours:literal : $minutes:literal : $seconds:literal : $frames:literal) {
    Time {
        hours: $hours,
        minutes: $minutes,
        seconds: $seconds,
        frames: $frames,
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, time::Duration};

    use super::{time, Time};

    #[test]
    fn test_time_parse() {
        assert_eq!(
            "00:00:00:00".parse::<Time>().unwrap(),
            Time {
                hours: 0,
                minutes: 0,
                seconds: 0,
                frames: 0,
            }
        );

        assert_eq!(
            "00:15".parse::<Time>().unwrap(),
            Time {
                hours: 0,
                minutes: 0,
                seconds: 0,
                frames: 15,
            }
        );

        assert_eq!(
            "12".parse::<Time>().unwrap(),
            Time {
                hours: 0,
                minutes: 0,
                seconds: 0,
                frames: 12,
            }
        );

        assert_eq!(
            "00:05:00:00".parse::<Time>().unwrap(),
            Time {
                hours: 0,
                minutes: 5,
                seconds: 0,
                frames: 0,
            }
        );
    }

    #[test]
    fn test_time_macro() {
        assert_eq!(time!(00:00:12:00), Time::from_str("00:00:12:00").unwrap());
    }

    #[test]
    fn test_as_frames() {
        assert_eq!(time!(00:00:00:00).as_frames(24.0), 0);
        assert_eq!(time!(00:00:10:15).as_frames(30.0), 315);
        assert_eq!(time!(12:34:56:78).as_frames(24.0), 1087182);
    }

    #[test]
    fn test_from_duration() {
        assert_eq!(
            Time::from_duration(Duration::from_secs(0), 24.0),
            time!(00:00:00:00)
        );
        assert_eq!(
            Time::from_duration(Duration::from_secs(10), 30.0),
            time!(00:00:10:00)
        );
        assert_eq!(
            Time::from_duration(Duration::from_millis(500), 24.0),
            time!(00:00:00:12)
        );
    }
}
