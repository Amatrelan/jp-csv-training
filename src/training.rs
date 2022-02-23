use log::{error, info};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::csv_struct::{read_data, CSVStruct};

#[derive(Debug)]
pub struct Training {
    pub training_data: Vec<TrainingData>,
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
    Japanese,
    English,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Done {
    pub japanese: bool,
    pub english: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrainingData {
    pub csv_data: CSVStruct,
    pub done: Done,
}

impl TrainingData {
    pub fn from_csvdata(csv_data: CSVStruct) -> Self {
        Self { csv_data, done: Done::default() }
    }

    pub fn mark_done(&mut self, lang: Language) -> Result<(), &'static str> {
        match lang {
            Language::Japanese => {
                if self.done.japanese {
                    return Err("Already done");
                }
                self.done.japanese = true
            }
            Language::English => {
                if self.done.english {
                    return Err("Already done");
                }
                self.done.english = true
            }
        }

        Ok(())
    }

    pub fn is_done(&self) -> bool {
        self.done.english && self.done.japanese
    }
}

impl Training {
    pub fn from_csv(path: String) -> Training {
        info!("Reading file to memory: {}", path);

        let training_data = match read_data(&path) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to read csv file ({}) with error code: {}", path, e);
                panic!("{}", e);
            }
        };

        let mut tmp = vec![];

        for each in training_data {
            tmp.push(TrainingData::from_csvdata(each));
        }

        Training { training_data: tmp }
    }

    pub fn get_random(&mut self) -> Option<(TrainingData, Language)> {
        if self.training_data.is_empty() {
            return None;
        }

        let mut rng = thread_rng();
        let selected = self.training_data.choose(&mut rng).cloned().unwrap();

        let mut language = Language::Japanese;

        self.training_data.iter_mut().for_each(|x| {
            if x == &selected {
                language = match rand::random::<bool>() {
                    true => {
                        if x.done.japanese {
                            Language::English
                        } else {
                            Language::Japanese
                        }
                    }
                    false => {
                        if x.done.english {
                            Language::Japanese
                        } else {
                            Language::English
                        }
                    }
                };
            }
        });

        Some((selected, language))
    }

    pub fn len(&self) -> usize {
        self.training_data.len()
    }

    pub fn mark_done(&mut self, trained: &TrainingData, lang: Language) {
        self.training_data
            .iter_mut()
            .find(|x| x == &trained)
            .expect("No such element exists in training_data")
            .mark_done(lang)
            .expect("Failed to mark done");
        self.training_data.retain(|x| !x.is_done());
    }
}
