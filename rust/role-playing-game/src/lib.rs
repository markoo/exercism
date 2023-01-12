
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mut new_player = Player {
                health: 100,
                mana: self.mana,
                level: self.level,
            };
            if self.level >= 10 {
                new_player.mana = Some(100);
            }
            Some(new_player)
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_none() {
            if self.health <= mana_cost {
                self.health = 0;
            } else {
                self.health -= mana_cost;
            }
            0
        } else if self.mana < Some(mana_cost) {
            0
        } else {
            self.mana = Some(self.mana.unwrap() - mana_cost);
            mana_cost * 2
        }
    }
}
