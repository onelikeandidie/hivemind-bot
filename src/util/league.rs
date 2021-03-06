use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LeagueResponse {
    pub abilities: LeaguePlayerAbilities,
    pub level: i32,
}

impl Default for LeagueResponse {
    fn default() -> Self {
        Self { abilities: Default::default(), level: 0 }
    }
}

impl std::fmt::Debug for LeagueResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeagueResponse").field("abilities", &self.abilities).field("level", &self.level).finish()
    }
}

impl std::cmp::PartialEq for LeagueResponse {
    fn eq(&self, other: &Self) -> bool {
        self.abilities == other.abilities && self.level == other.level
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LeaguePlayerAbilities {
    pub Q: LeagueAbility,
    pub W: LeagueAbility,
    pub E: LeagueAbility,
    pub R: LeagueAbility,
    pub Passive: LeagueAbility,
}

impl LeaguePlayerAbilities {
    pub fn check_used_points(& self) -> i32 {
        let mut i = 0;
        if let Some(q_level) = self.Q.abilityLevel {i += q_level};
        if let Some(w_level) = self.W.abilityLevel {i += w_level};
        if let Some(e_level) = self.E.abilityLevel {i += e_level};
        if let Some(r_level) = self.R.abilityLevel {i += r_level};
        i
    }
}

impl Default for LeaguePlayerAbilities {
    fn default() -> Self {
        Self { Q: Default::default(), W: Default::default(), E: Default::default(), R: Default::default(), Passive: Default::default() }
    }
}

impl std::fmt::Debug for LeaguePlayerAbilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeaguePlayerAbilities").field("Q", &self.Q).field("W", &self.W).field("E", &self.E).field("R", &self.R).field("Passive", &self.Passive).finish()
    }
}

impl std::cmp::PartialEq for LeaguePlayerAbilities {
    fn eq(&self, other: &Self) -> bool {
        self.Q == other.Q && self.W == other.W && self.E == other.E && self.R == other.R && self.Passive == other.Passive
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LeagueAbility {
    pub abilityLevel: Option<i32>
}

impl Default for LeagueAbility {
    fn default() -> Self {
        Self { abilityLevel: None }
    }
}

impl std::fmt::Debug for LeagueAbility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeagueAbility").field("abilityLevel", &self.abilityLevel).finish()
    }
}

impl std::cmp::PartialEq for LeagueAbility {
    fn eq(&self, other: &Self) -> bool {
        match self.abilityLevel {
            Some(self_level) => {
                if let Some(other_level) = other.abilityLevel {
                    return self_level == other_level;
                } else {
                    return false;
                }
            },
            None => {
                if let None = other.abilityLevel {
                    return true;
                } else {
                    return false;
                }
            },
        }
        //self.abilityLevel == other.abilityLevel
    }
}