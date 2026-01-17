pub struct GameConstants {
    skill_names: Vec<&'static str>,
}

impl Default for GameConstants {
    fn default() -> Self {
        GameConstants {
            skill_names: include_str!("../assets/skill_names.txt").lines().collect(),
        }
    }
}
