pub struct SequenceMatcher<'a> {
    shorter: &'a str,
    longer: &'a str
}

impl<'a> SequenceMatcher<'a> {
    pub fn new(s1: &'a str, s2: &'a str) -> SequenceMatcher<'a> {
        SequenceMatcher{ shorter:s1, longer:s2 }
    }
    pub fn get_matching_blocks(&self) -> Vec<(u32,u32,u32)> {
        let result: Vec<(u32, u32, u32)> = Vec::new();

        result
    }
    pub fn ratio(&self) -> f32 {
        0.0
    }
}