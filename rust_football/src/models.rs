#[derive(Debug)]
pub struct PlayerCA {
    pub player: String,
    pub ninety_mins: String,
    pub gca90: String,
    pub sca90: String
}

impl PlayerCA {
    pub fn ninety_mins(&self) -> f64 {
        self.ninety_mins.parse().unwrap_or(0.0)
    }
    
    pub fn sca90(&self) -> f64 {
        self.sca90.parse().unwrap_or(0.0)
    }
    
    pub fn gca90(&self) -> f64 {
        self.gca90.parse().unwrap_or(0.0)
    }
    
    pub fn is_regular_starter(&self) -> bool {
        self.ninety_mins() > 0.7
    }
    
    pub fn creativity_score(&self) -> f64 {
        self.sca90() + self.gca90()
    }
}

#[derive(Debug)]
pub struct PlayerPass {
    pub player: String,
    pub ninety_mins: String,
    pub exp_assist: String,
    pub key_passes: String,
    pub penalty_area_passes: String,
    pub penalty_area_crosses: String
}

impl PlayerPass {
    pub fn ninety_mins(&self) -> f64 {
        self.ninety_mins.parse().unwrap_or(0.0)
    }
    
    pub fn x_a(&self) -> f64 {
        self.exp_assist.parse().unwrap_or(0.0)
    }
    
    pub fn key_passes(&self) -> f64 {
        self.key_passes.parse().unwrap_or(0.0)
    }

    pub fn penalty_area_passes(&self) -> f64 {
        self.penalty_area_passes.parse().unwrap_or(0.0)
    }

    pub fn penalty_area_crosses(&self) -> f64 {
        self.penalty_area_crosses.parse().unwrap_or(0.0)
    }

    pub fn is_regular_starter(&self) -> bool {
        self.ninety_mins() > 0.7
    }

    pub fn pass_danger(&self) -> f64 {
        (self.x_a() * 2.0) +
        (self.key_passes() * 1.5) +
        (self.penalty_area_passes() * 1.2) +
        (self.penalty_area_crosses() * 1.0)
    }

    pub fn pass_danger_per_90(&self) -> f64 {
        let total_danger = self.pass_danger();
        if self.ninety_mins() > 0.0 {
            total_danger / self.ninety_mins()
        } else {
            0.0
        }
    }
}

#[derive(Debug)]
pub struct PlayerRecommendation {
    pub player: String,
    pub overall_score: f64,
    pub creativity: f64,
    pub passing_threat: f64,
}

impl PlayerRecommendation {
    pub fn new(ca_player: &PlayerCA, pass_player: &PlayerPass) -> Option<Self> {
        if ca_player.player == pass_player.player && 
           ca_player.is_regular_starter() && 
           pass_player.is_regular_starter() {
            
            let creativity = ca_player.creativity_score();
            let passing_threat = pass_player.pass_danger_per_90();
            
            if creativity > 0.0 && passing_threat > 0.0 {
                let overall_score = (creativity * 0.6) + (passing_threat * 0.4);
                
                Some(PlayerRecommendation {
                    player: ca_player.player.clone(),
                    overall_score,
                    creativity,
                    passing_threat,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}