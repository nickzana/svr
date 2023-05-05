use crate::Error;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(rename = "ytdl-path")]
    pub youtube_dl_path: Option<PathBuf>,
    #[serde(rename = "cookies")]
    pub cookie_jar_path: Option<PathBuf>,
    pub output: PathBuf,
    pub streams: Vec<Stream>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Stream {
    #[serde(
        rename = "youtube-dl",
        alias = "yt-dl",
        alias = "ytdl",
        alias = "yt-dlp",
        alias = "ytdlp"
    )]
    YoutubeDl {
        url: String,
        #[serde(with = "humantime_serde")]
        frequency: Duration,
        quality: Option<String>,
        // Relative to Config::output. If `None`, will be downloaded directly to Config::output
        subpath: Option<PathBuf>,
        // Overrides Config::output, still respects subpath (becomes relative to new output)
        output: Option<PathBuf>,
        // YouTube only
        #[serde(default)]
        live_from_start: bool,
    },
}

impl Stream {
    /// # Errors
    /// Should only return an error on an unrecoverable error, things like
    /// e.g. a stream not being live or a video not being found are
    /// to be handled internally.
    pub async fn watch(&self, config: &Config) -> Result<(), Error> {
        match self {
            Stream::YoutubeDl {
                url,
                frequency,
                quality,
                subpath,
                output,
                live_from_start,
            } => {
                let mut dl = YoutubeDl::new(url);
                dl.download(true);

                if let Some(path) = &config.youtube_dl_path {
                    dl.youtube_dl_path(path);
                }

                if let Some(quality) = quality {
                    dl.format(quality);
                }

                let mut output_dir = output.clone().unwrap_or(config.output.clone());
                if let Some(subpath) = subpath {
                    output_dir = output_dir.join(subpath);
                }
                dl.output_directory(output_dir.to_string_lossy());

                if let Some(path) = &config.cookie_jar_path {
                    dl.cookies(path.to_string_lossy());
                }

                if *live_from_start {
                    dl.extra_arg("--live-from-start");
                }

                loop {
                    match dl.run_async().await {
                        Ok(YoutubeDlOutput::SingleVideo(video)) => {
                            println!(
                                "Successfully downloaded {} `{url:#?}`: {video:#?}",
                                if Some(true) == video.is_live {
                                    "livestream"
                                } else {
                                    "video"
                                }
                            );
                        }
                        Ok(YoutubeDlOutput::Playlist(playlist)) => {
                            println!("Successfully downloaded playlist `{url:#?}`: {playlist:#?}");
                        }
                        Err(e) => {
                            let err = if let youtube_dl::Error::ExitCode { stderr, .. } = e {
                                stderr
                            } else if let youtube_dl::Error::Io(ioerr) = e {
                                ioerr.to_string()
                            } else {
                                e.to_string()
                            };
                            eprintln!("Error with video at URL `{url:#?}`: {err}");
                        }
                    }

                    tokio::time::sleep(*frequency).await;
                }
            }
        }
    }
}
