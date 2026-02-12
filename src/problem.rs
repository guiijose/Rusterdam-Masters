#![allow(dead_code)]

use std::collections::HashMap;
use crate::course::Course;

pub struct Problem {
    pub courses: Vec<Course>,

    // key: number of period | value: vector indexes in courses vector
    pub courses_by_period: HashMap<u8,Vec<usize>>,

    // key: string of group | value: vector indexes in courses vector
    pub courses_by_group: HashMap<String,Vec<usize>>,
}

impl Problem {

}
