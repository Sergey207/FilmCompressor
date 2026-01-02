use crate::app::ffmpeg_manager::compress_settings::CompressSettings;
use serde_json::Value;
use std::fmt::Display;
use std::io::{Error, ErrorKind};
use std::mem::discriminant;
use std::path::PathBuf;
use std::process::Command;

#[derive(Clone, Eq, PartialEq)]
pub struct VideoData {
    pub resolution: (u64, u64),
}

#[derive(Clone, Eq, PartialEq)]
pub struct AudioData {
    pub title: Option<String>,
    pub channels: u64,
    pub language: Option<String>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct SubtitleData {
    pub title: Option<String>,
    pub language: Option<String>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum StreamType {
    Video(VideoData),
    Audio(AudioData),
    Subtitle(SubtitleData),
    Attachment,
}

impl StreamType {
    pub fn to_index(&self) -> usize {
        match self {
            StreamType::Video(_) => 0,
            StreamType::Audio(_) => 1,
            StreamType::Subtitle(_) => 2,
            StreamType::Attachment => 3,
        }
    }
}

impl Display for StreamType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            StreamType::Video(_) => "v",
            StreamType::Audio(_) => "a",
            StreamType::Subtitle(_) => "s",
            StreamType::Attachment => "t",
        })
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Stream {
    pub stream_type: StreamType,
    pub codec_name: String,
    pub default: bool,
}

impl Display for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self.stream_type {
            StreamType::Video(video_data) => String::from(format!(
                "Video {} ({}:{})",
                self.codec_name, video_data.resolution.0, video_data.resolution.1
            )),
            StreamType::Audio(audio_data) => {
                let mut result = String::from(format!(
                    "Audio {} ({} ch)",
                    self.codec_name, audio_data.channels
                ));
                if let Some(title) = &audio_data.title {
                    result += &format!(" {}", title);
                }
                if let Some(language) = &audio_data.language {
                    result += &format!(" {}", language);
                }
                result
            }
            StreamType::Subtitle(subtitle_data) => {
                let mut result = String::from(format!("Subtitle {}", self.codec_name));
                if let Some(title) = &subtitle_data.title {
                    result += &format!(" {}", title);
                }
                if let Some(language) = &subtitle_data.language {
                    result += &format!(" {}", language);
                }
                result
            }
            StreamType::Attachment => String::from("Attachment"),
        };
        write!(f, "{}", str)
    }
}

pub struct InputFile {
    pub path: PathBuf,
    pub sources: Vec<Stream>,
}

impl InputFile {
    fn from_path(path: PathBuf) -> Self {
        let sources = Self::process_path(&path).unwrap_or(Vec::new());
        Self { path, sources }
    }

    fn process_path(path: &PathBuf) -> Result<Vec<Stream>, Error> {
        if !path.exists() {
            return Err(ErrorKind::NotFound.into());
        }
        let output = Command::new("ffprobe")
            .arg("-show_streams")
            .arg("-output_format")
            .arg("json")
            .arg(path.to_str().expect("Failed to convert path to string"))
            .output()?;
        if !output.status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                "ffprobe exited with non-zero status code",
            ));
        }

        let json_string =
            String::from_utf8(output.stdout).expect("Failed to convert output to string");
        let json_data: Value = serde_json::from_str(&json_string)?;

        let mut result = vec![];
        for stream in json_data["streams"].as_array().unwrap() {
            let new_stream = Stream {
                stream_type: match stream["codec_type"].as_str().unwrap() {
                    "video" => StreamType::Video(VideoData {
                        resolution: (
                            stream["width"].as_u64().unwrap(),
                            stream["height"].as_u64().unwrap(),
                        ),
                    }),
                    "audio" => StreamType::Audio(AudioData {
                        title: match stream["tags"]["title"].as_str() {
                            Some(title) => Some(title.to_string()),
                            None => None,
                        },
                        channels: stream["channels"].as_u64().unwrap(),
                        language: match stream["tags"]["language"].as_str() {
                            Some(language) => Some(language.to_string()),
                            None => None,
                        },
                    }),
                    "subtitle" => StreamType::Subtitle(SubtitleData {
                        title: match stream["tags"]["title"].as_str() {
                            Some(title) => Some(title.to_string()),
                            None => None,
                        },
                        language: match stream["tags"]["language"].as_str() {
                            Some(language) => Some(language.to_string()),
                            None => None,
                        },
                    }),
                    "attachment" => StreamType::Attachment,
                    &_ => {
                        continue;
                    }
                },
                codec_name: stream["codec_name"].as_str().unwrap().to_string(),
                default: stream["disposition"]["default"].as_u64().unwrap_or(0) == 1,
            };
            result.push(new_stream);
        }
        Ok(result)
    }
}

pub enum FfmpegStreamFiles {
    All,
    Partial(Vec<usize>),
}

impl Display for FfmpegStreamFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfmpegStreamFiles::All => write!(f, ""),
            FfmpegStreamFiles::Partial(files) => {
                let files = files.iter().map(|f| f.to_string()).collect::<Vec<String>>();
                write!(f, " ({})", files.join("|"))
            }
        }
    }
}

pub struct FfmpegStreamSettings {
    pub stream: Stream,
    pub files: FfmpegStreamFiles,
    pub enabled: bool,
    pub default: bool,
}

impl Display for FfmpegStreamSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result += if self.enabled { "[X] " } else { "[ ] " };
        if self.default {
            result += "[D] ";
        }
        result += &self.stream.to_string();
        result += &self.files.to_string();
        write!(f, "{}", result)
    }
}

pub struct FfmpegManager {
    pub input_files: Vec<InputFile>,
    pub stream_settings: Vec<FfmpegStreamSettings>,
    pub compress_settings: CompressSettings,
}

impl FfmpegManager {
    pub fn add_file(&mut self, path: PathBuf) {
        let input_file = InputFile::from_path(path);
        if !input_file.sources.is_empty() {
            self.input_files.push(input_file);
            self.update_stream_settings();
        }
    }

    pub fn add_folder(&mut self, path: PathBuf) {
        if !path.is_dir() {
            return;
        }
        let mut paths: Vec<_> = path
            .read_dir()
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        paths.sort();
        for path in paths {
            self.add_file(path);
        }
    }

    pub fn add_path(&mut self, path: PathBuf) {
        if path.exists() {
            if path.is_file() {
                self.add_file(path);
            } else {
                self.add_folder(path);
            }
        }
    }
    pub fn update_stream_settings(&mut self) {
        self.stream_settings.clear();
        let mut sources = Vec::new();
        for input_file in self.input_files.iter() {
            for source in input_file.sources.iter() {
                if !sources.contains(source) {
                    self.stream_settings.push(FfmpegStreamSettings {
                        stream: source.clone(),
                        files: FfmpegStreamFiles::All,
                        enabled: true,
                        default: source.default,
                    });
                    sources.push(source.clone());
                }
            }
        }
        for stream_settings in &mut self.stream_settings {
            let mut files = Vec::new();
            for (i, input_file) in self.input_files.iter().enumerate() {
                if input_file.sources.contains(&stream_settings.stream) {
                    files.push(i + 1);
                }
            }
            if files.len() != self.input_files.len() {
                stream_settings.files = FfmpegStreamFiles::Partial(files);
            }
        }
    }

    pub fn toggle_default(&mut self, index: usize) {
        let new_value = !self.stream_settings[index].default;
        let stream_type = discriminant(&self.stream_settings[index].stream.stream_type);
        self.stream_settings
            .iter_mut()
            .filter(|s| discriminant(&s.stream.stream_type) == stream_type)
            .for_each(|s| s.default = false);
        self.stream_settings[index].default = new_value;
    }

    pub fn get_command_template(&self) -> String {
        let mut result = vec!["ffmpeg".to_string()];
        result.extend(self.compress_settings.get_init_arguments());
        result.push("<input file> <streams>".to_string());
        result.extend(self.compress_settings.get_compress_arguments());
        result.push("<output file>".to_string());
        result.join(" ")
    }

    fn get_command_streams(&self, input_file: &InputFile) -> Vec<String> {
        let mut result = Vec::new();
        let mut source_indexes = [0; 4];
        input_file.sources.iter().for_each(|source| {
            let stream_setting = self
                .stream_settings
                .iter()
                .find(|ss| ss.stream == *source)
                .unwrap();
            let source_index = stream_setting.stream.stream_type.to_index();
            if stream_setting.enabled {
                result.extend(vec![
                    "-map".to_string(),
                    format!("0:{}:{}", source.stream_type, source_indexes[source_index]),
                ]);
            }
            if stream_setting.default {
                result.extend(vec![
                    format!("-disposition:{}", source.stream_type),
                    (source_indexes[source_index] + 1).to_string(),
                ]);
            }
            source_indexes[source_index] += 1;
        });
        result
    }

    pub fn get_command(&self, input_file: &InputFile, output_path: &PathBuf) -> Vec<String> {
        let mut result = vec![];
        result.extend(self.compress_settings.get_init_arguments());
        result.push("-i".to_string());
        result.push(
            input_file
                .path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
        );
        result.extend(self.get_command_streams(input_file));
        result.extend(self.compress_settings.get_compress_arguments());
        result.push(output_path.to_string_lossy().to_string());
        result
    }
}

impl Default for FfmpegManager {
    fn default() -> Self {
        Self {
            input_files: Vec::new(),
            stream_settings: Vec::new(),
            compress_settings: CompressSettings::default(),
        }
    }
}
