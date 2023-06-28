# video-presenter [![Build](https://github.com/Basicprogrammer10/video-presenter/actions/workflows/build.yml/badge.svg)](https://github.com/Basicprogrammer10/video-presenter/actions/workflows/build.yml)

Uses cuepoints put into a video edited with [Premiere Pro](https://www.adobe.com/products/premiere.html) or [After Effects](https://www.adobe.com/products/aftereffects.html).
Then when playing back it will wait at the cuepoints for the space button to be pressed.
This will let use use normal videos for presentations allowing for more advanced graphics and animations, while still allowing you to keep perfect timing.
Video playback is handled through [libmpv](https://github.com/mpv-player/mpv), so you can use like almost any video format.

## Player Keybinds

| Key              | Action                 |
| ---------------- | ---------------------- |
| <kbd>Space</kbd> | Continue / advance cue |
| <kbd>Right</kbd> | Seek to next cue       |
| <kbd>Left</kbd>  | Seek to last cue       |
| <kbd>P</kbd>     | Pause / unpause        |
| <kbd>></kbd>     | Jump one frame forward |
| <kbd><</kbd>     | Jump one frame back    |

## Command Line Usage

`video-presenter [OPTIONS] <MEDIA_FILE> <CUE_FILE>`

| Option                | Description                                                                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `--help`, `-h`        | Prints help                                                                                                                                     |
| `--version`, `-V`     | Prints version information                                                                                                                      |
| `--mpv_setting`, `-m` | Lets you pass [a setting](https://mpv.io/manual/stable/#property-list) to the mpv backend, can be used multiple times. (Ex: `-m setting=value`) |
| `--audio`, `-a`       | Enables audio output. (Disabled by default)                                                                                                     |

## How to make a cue file

I use Premiere Pro and After Effects, so those are the only ones I can give instructions for.
For each place you want a cue point, add a marker to the root of the video.
If in Premiere, make sure the marker type is `Flash Cue Point`.

Then to export to a file, in Premiere, go to `File › Export › Markers`, then choose `.csv` as the format.
If using After Effects, you can use [Marker Batch Editor Script](https://aescripts.com/marker-batch-editor) (its free) with this output formatter: `,,[time],[time],[markerDuration],Cue Point\n` to make the file.
