use lazy_static::lazy_static;
use std::collections::HashMap;

use super::character::Character;

lazy_static! {
    pub static ref CHARACTERS: HashMap<char, Character> = {
        let mut m = HashMap::new();

        m.insert(
            '0',
            Character::new(vec![
                vec![1, 1, 1, 0],
                vec![1, 0, 1, 0],
                vec![1, 0, 1, 0],
                vec![1, 0, 1, 0],
                vec![1, 1, 1, 0],
            ]),
        );

        m.insert(
            '1',
            Character::new(vec![
                vec![1, 0],
                vec![1, 0],
                vec![1, 0],
                vec![1, 0],
                vec![1, 0],
            ]),
        );

        m.insert(
            '2',
            Character::new(vec![
                vec![1, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 1, 1, 0],
            ]),
        );

        m.insert(
            '3',
            Character::new(vec![
                vec![1, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![1, 1, 1, 0],
            ]),
        );

        m.insert(
            '4',
            Character::new(vec![
                vec![1, 0, 1, 0],
                vec![1, 0, 1, 0],
                vec![1, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
            ]),
        );

        m.insert(
            '5',
            Character::new(vec![
                vec![1, 1, 1, 0],
                vec![1, 0, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![1, 1, 1, 0],
            ]),
        );

        m.insert(
            '6',
            Character::new(vec![
                vec![1, 1, 1, 0],
                vec![1, 0, 0, 0],
                vec![1, 1, 1, 0],
                vec![1, 0, 1, 0],
                vec![1, 1, 1, 0],
            ]),
        );

        m.insert(
            '7',
            Character::new(vec![
                vec![1, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
            ]),
        );

        m.insert(
            '8',
            Character::new(vec![
                vec![1, 1, 1, 0],
                vec![1, 0, 1, 0],
                vec![1, 1, 1, 0],
                vec![1, 0, 1, 0],
                vec![1, 1, 1, 0],
            ]),
        );

        m.insert(
            '9',
            Character::new(vec![
                vec![1, 1, 1, 0],
                vec![1, 0, 1, 0],
                vec![1, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
            ]),
        );

        m
    };
}
