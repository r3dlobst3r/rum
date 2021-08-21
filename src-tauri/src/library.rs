use crate::config::Config;
use crate::database::DatabaseFetcher;
use crate::util::ChadError;
use serde::Serialize;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use titlecase::titlecase;

#[derive(Serialize, Clone, Debug)]
pub struct Game {
    id: usize,
    name: String,
    executable_path: PathBuf,
    banner_path: Option<PathBuf>,
    banner: Option<String>,
    data_path: PathBuf,
    log_file: PathBuf,
    config_file: PathBuf,
}

fn load_banner(banner_path: &Path) -> Option<String> {
    std::fs::read(banner_path)
        .ok()
        .map(|b| base64::encode(b))
        .map(|b64| format!("data:image/png;base64,{}", b64))
}

impl Game {
    pub fn new(config: &Config, id: usize, executable_path: PathBuf) -> Self {
        let slug: String = executable_path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .into();
        let mut name = slug.clone();
        name = name.replace(".", " ");
        name = name.replace("_", " ");
        name = name.replace("-", " ");
        name = titlecase(&name).trim().into();

        let data_path = config.data_path().join("library").join(slug);
        let _ = std::fs::create_dir_all(&data_path);

        let banner_path = if data_path.join("banner.png").exists() {
            Some(data_path.join("banner.png"))
        } else {
            None // TODO Fetch banner
        };

        let banner = banner_path.as_ref().and_then(|p| load_banner(&p));

        let config_file = data_path.join("game.yaml");
        let log_file = executable_path.parent().unwrap().join("chad.log");

        Self {
            id,
            name,
            executable_path,
            banner_path,
            banner,
            data_path,
            log_file,
            config_file,
        }
    }

    pub fn executable_path(&self) -> &Path {
        &self.executable_path
    }

    pub async fn get_banner(
        &mut self,
        fetcher: &DatabaseFetcher,
    ) -> Result<(), ChadError> {
        if let Ok(banner_path) = fetcher.find_banner(&self.name).await {
            let target = format!(
                "https://gitlab.com/chad-productions/chad_launcher_banners/-/raw/master/{}",
                banner_path
            );
            let response = reqwest::get(target).await?;
            let content = response.text().await?;
            std::io::copy(
                &mut content.as_bytes(),
                &mut std::fs::File::create(self.data_path.join("banner.png"))?,
            )?;
            self.banner_path = Some(self.data_path.join("banner.png"));
            self.banner = self.banner_path.as_ref().and_then(|p| load_banner(&p));
        }

        Ok(())
    }

    pub fn launch(&self) -> Result<Box<dyn Read>, ChadError> {
        let child = Command::new(&self.executable_path)
            .current_dir(self.executable_path.parent().unwrap())
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(Box::new(child.stdout.unwrap()))
    }
}

#[derive(Debug, Default)]
pub struct LibraryFetcher {
    games: Vec<Game>,
}

impl LibraryFetcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_games(&mut self, config: &Config) {
        self.games = Vec::new();
        let mut id = 0;

        for library_path in config.library_paths() {
            if let Ok(entries) = library_path.read_dir() {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                if entry.path().join("start.sh").exists() {
                                    self.games.push(Game::new(
                                        &config,
                                        id,
                                        entry.path().join("start.sh"),
                                    ));
                                    id += 1;
                                } else if entry.path().join("start").exists() {
                                    self.games.push(Game::new(
                                        &config,
                                        id,
                                        entry.path().join("start"),
                                    ));
                                    id += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn iter_games<'a>(&'a self) -> impl Iterator<Item = &'a Game> {
        self.games.iter()
    }

    pub fn get_games<'a>(&'a self) -> Vec<&'a Game> {
        self.iter_games().collect()
    }

    pub fn get_games_cloned(&self) -> Vec<Game> {
        self.iter_games().cloned().collect()
    }

    pub fn get_game<'a>(&'a self, index: usize) -> Option<&'a Game> {
        self.games.get(index)
    }
}

