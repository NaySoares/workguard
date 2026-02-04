use chrono::NaiveTime;

#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    HardPause, // não conta como trabalho
    SoftPause, // conta como trabalho, mas avisa
}

#[derive(Debug, Clone)]
pub struct TimeBlock {
    pub label: String,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub block_type: BlockType,
}

impl TimeBlock {
    pub fn contains(&self, time: NaiveTime) -> bool {
        time >= self.start && time < self.end
    }
}

#[derive(Debug, Clone)]
pub struct Schedule {
    pub blocks: Vec<TimeBlock>,
}

impl Schedule {
    pub fn get_active_block(&self, now: NaiveTime) -> Option<&TimeBlock> {
        self.blocks.iter().find(|block| block.contains(now))
    }
}