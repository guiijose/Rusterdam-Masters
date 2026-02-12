#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Course {
    pub name: String,
    pub group: String,
    pub period: u8,
    pub ects: u8,
    pub score: f32,
}

impl Course {
    pub fn new(
        name: String,
        group: String,
        period: u8,
        ects: u8,
        score: f32
    ) -> Result<Self,String> {

        if !(1..=6).contains(&period) {
            return Err(format!("Period {} is invalid, must be integer between 1-6", period));
        }

        if !(1.0..=10.0).contains(&score) {
            return Err(format!("Score {} is invalid, must be between 1-10", score));
        }

        if name.is_empty() {
            return Err("Name of course cannot be empty".to_string());
        }

        if group.is_empty() {
            return Err("Group of course cannot be empty".to_string());
        }

        // ects is unsigned therefore no is positive check if needed

        Ok(Course { name, group, period, ects, score })
    }

    pub fn summary(&self) -> String {
        format!("{} ({}, P{}, {} EC, score {})", self.name, self.group, self.period, self.ects, self.score)
    }
}
