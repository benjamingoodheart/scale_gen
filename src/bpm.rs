use rand::random_range;
pub struct BPM {
    pub floor_bpm: i32,
    pub ceiling_bpm: i32,
}

impl BPM {
    pub fn new()->BPM{
        BPM{
            floor_bpm: 60,
            ceiling_bpm: 225,
        }
    }
    pub fn get_random_bpm(&self)->i32{
        let my_bpm = random_range(self.floor_bpm..self.ceiling_bpm);
        my_bpm
    }
    fn set_floor(&mut self,floor_val: i32)->Result<(), &str>{
        //TODO: add guard rails for higher than ceiling
        self.floor_bpm = floor_val;
        Ok(())
    }
    fn set_ceiling(&mut self, ceiling_val: i32)->Result<(), &str>{
       //TODO: add guard rails for lower than floor
        self.ceiling_bpm = ceiling_val;
        Ok(())
    }
}
