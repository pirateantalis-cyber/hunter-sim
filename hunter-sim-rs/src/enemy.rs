//! Enemy and Boss implementations - Updated to match CIFI Tools formulas

use crate::config::HunterType;
use rand::Rng;

/// A regular enemy in combat
#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: String,
    pub hp: f64,
    pub max_hp: f64,
    pub power: f64,
    pub base_power: f64,  // Store base power for enrage calculations
    pub regen: f64,
    pub damage_reduction: f64,
    pub evade_chance: f64,
    pub effect_chance: f64,  // Added: enemy effect chance (starts at stage 300)
    pub special_chance: f64,
    pub special_damage: f64,
    pub speed: f64,
    pub base_speed: f64,  // Store base speed for enrage calculations
    pub is_boss: bool,
    pub is_stunned: bool,
    pub stun_end_time: f64,
    // Boss-specific
    pub enrage_stacks: i32,
    pub has_secondary: bool,
    pub speed2: f64,
    pub base_speed2: f64,
}

impl Enemy {
    /// CIFI stage scaling function for Borge/Ozzy (multiWasm)
    fn multi_wasm(stage: i32) -> f64 {
        let s = stage as f64;
        let mut result = 1.0;
        result += (s - 149.0).max(0.0) * 0.006;
        result += (s - 199.0).max(0.0) * 0.006;
        result += (s - 249.0).max(0.0) * 0.006;
        result += (s - 299.0).max(0.0) * 0.006;
        result += (s - 309.0).max(0.0) * 0.003;
        result += (s - 319.0).max(0.0) * 0.003;
        result += (s - 329.0).max(0.0) * 0.004;
        result += (s - 339.0).max(0.0) * 0.004;
        result += (s - 349.0).max(0.0) * 0.005;
        result += (s - 359.0).max(0.0) * 0.005;
        result += (s - 369.0).max(0.0) * 0.006;
        result += (s - 379.0).max(0.0) * 0.006;
        result += (s - 389.0).max(0.0) * 0.007;
        result = result.max(1.0);
        result *= 1.01_f64.powf((s - 350.0).max(0.0));
        result
    }
    
    /// CIFI stage scaling function for Knox (f_o)
    fn knox_scaling(stage: i32) -> f64 {
        let s = stage as f64;
        let mut result = (s - 49.0) * 0.006 + 1.0;
        result += (s - 99.0).max(0.0) * 0.006;
        result += (s - 119.0).max(0.0) * 0.01;
        result += (s - 129.0).max(0.0) * 0.008;
        result += (s - 139.0).max(0.0) * 0.006;
        result += (s - 149.0).max(0.0) * 0.006;
        result += (s - 159.0).max(0.0) * 0.006;
        result += (s - 169.0).max(0.0) * 0.006;
        result += (s - 179.0).max(0.0) * 0.006;
        result += (s - 189.0).max(0.0) * 0.006;
        result += (s - 199.0).max(0.0) * 0.006;
        result += (s - 219.0).max(0.0) * 0.02;
        result += (s - 249.0).max(0.0) * 0.006;
        result += (s - 299.0).max(0.0) * 0.006;
        result += (s - 309.0).max(0.0) * 0.003;
        result += (s - 319.0).max(0.0) * 0.02;
        result += (s - 329.0).max(0.0) * 0.004;
        result += (s - 339.0).max(0.0) * 0.004;
        result += (s - 349.0).max(0.0) * 0.005;
        result += (s - 359.0).max(0.0) * 0.005;
        result += (s - 369.0).max(0.0) * 0.006;
        result += (s - 379.0).max(0.0) * 0.006;
        result += (s - 389.0).max(0.0) * 0.007;
        result.max(1.0)
    }

    /// Create a regular enemy for a given stage - using CIFI formulas
    pub fn new(index: i32, stage: i32, hunter_type: HunterType) -> Self {
        let (hp, power, regen, special_chance, special_damage, dr, evade_chance, effect_chance, speed) = 
            Self::calculate_stats_cifi(stage, hunter_type, false);
        
        Self {
            name: format!("E{:>3}{:>3}", stage, index),
            hp,
            max_hp: hp,
            power,
            base_power: power,
            regen,
            damage_reduction: dr,
            evade_chance,
            effect_chance,
            special_chance: special_chance.min(0.25),  // Cap at 25%
            special_damage: special_damage.min(2.5),   // Cap at 250%
            speed,
            base_speed: speed,
            is_boss: false,
            is_stunned: false,
            stun_end_time: 0.0,
            enrage_stacks: 0,
            has_secondary: false,
            speed2: 0.0,
            base_speed2: 0.0,
        }
    }
    
    /// Create a boss for a given stage - using CIFI formulas
    pub fn new_boss(stage: i32, hunter_type: HunterType) -> Self {
        let (hp, power, regen, special_chance, special_damage, dr, evade_chance, effect_chance, speed) = 
            Self::calculate_stats_cifi(stage, hunter_type, true);
        
        // Calculate speed2 for bosses (secondary attack speed)
        let speed2 = if stage >= 200 {
            speed * 1.8  // Secondary attack is slower
        } else {
            0.0
        };
        
        Self {
            name: format!("B{:>3}", stage),
            hp,
            max_hp: hp,
            power,
            base_power: power,
            regen,
            damage_reduction: dr,
            evade_chance,
            effect_chance,
            special_chance: special_chance.min(0.30),
            special_damage: special_damage.min(5.0),
            speed,
            base_speed: speed,
            is_boss: true,
            is_stunned: false,
            stun_end_time: 0.0,
            enrage_stacks: 0,
            has_secondary: stage >= 200,
            speed2,
            base_speed2: speed2,
        }
    }
    
    /// Calculate enemy stats using CIFI formulas extracted from WASM
    fn calculate_stats_cifi(stage: i32, hunter_type: HunterType, is_boss: bool) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        // Returns: (hp, power, regen, special_chance, special_damage, dr, evade_chance, effect_chance, speed)
        let s = stage as f64;
        let d = ((stage - 1).max(0) as f64 / 100.0).floor() as i32;  // Boss cycles completed
        let d_f = d as f64;
        let is_stage_300 = stage == 300;
        
        match hunter_type {
            HunterType::Borge => {
                let f = Self::multi_wasm(stage);
                
                // HP: (stage * 4 + 9) * f * 2.85^d * boss_mult * stage300_mult
                let hp = (s * 4.0 + 9.0) * f * 2.85_f64.powf(d_f) 
                    * if is_boss { 90.0 } else { 1.0 }
                    * if is_stage_300 { 0.9 } else { 1.0 };
                
                // Power: (stage * 0.7 + 2.5) * f * 2.85^d * boss_mult * stage300_mult
                let power = (s * 0.7 + 2.5) * f * 2.85_f64.powf(d_f)
                    * if is_boss { 3.63 } else { 1.0 }
                    * if is_stage_300 { 0.9 } else { 1.0 };
                
                // Crit chance: stage * 0.0004 + 0.0322 + boss_bonus, capped at 0.25
                let special_chance = (s * 0.0004 + 0.0322 + if is_boss { 0.04 } else { 0.0 }).min(0.25);
                
                // Crit damage: stage * 0.008 + 1.212 + boss_bonus, capped at 2.5
                let special_damage = (s * 0.008 + 1.212 + if is_boss { 0.25 } else { 0.0 }).min(2.5);
                
                // Damage reduction (starts at stage 200)
                let dr = if stage >= 200 {
                    let base = 1.0 - (d_f - 2.0).max(0.0) * 0.02 + 0.04;
                    base - if is_boss { 0.05 } else { 0.0 }
                } else {
                    if is_boss { 0.95 } else { 1.0 }
                };
                // Convert DR to actual reduction (1.0 = no reduction, 0.95 = 5% reduction)
                let actual_dr = 1.0 - dr;
                
                // Evade (starts at stage 100)
                let evade = if stage >= 100 {
                    (d_f - 1.0).max(0.0) * 0.004 + 0.004
                } else { 0.0 };
                
                // Effect chance (starts at stage 300)
                let effect = if stage >= 300 {
                    (d_f - 3.0).max(0.0) * 0.01 + 0.04 + if is_boss { 0.04 } else { 0.0 }
                } else { 0.0 };
                
                // Regen: (stage-1) * 0.08 * f * 1.052^d * boss_mult * stage300_mult
                let regen = ((s - 1.0).max(0.0) * 0.08 * f * 1.052_f64.powf(d_f)).max(0.0)
                    * if is_boss { 1.92 } else { 1.0 }
                    * if is_stage_300 { 0.9 } else { 1.0 };
                
                // Speed: (4.526 - stage * 0.006) * boss_mult
                let speed = (4.526 - s * 0.006) * if is_boss { 2.42 } else { 1.0 };
                
                (hp, power, regen, special_chance, special_damage, actual_dr, evade, effect, speed)
            }
            HunterType::Ozzy => {
                let f = Self::multi_wasm(stage);
                
                // HP: (stage * 6 + 11) * f * 2.9^d * boss_mult * stage300_mult
                let hp = (s * 6.0 + 11.0) * f * 2.9_f64.powf(d_f)
                    * if is_boss { 48.0 } else { 1.0 }
                    * if is_stage_300 { 0.94 } else { 1.0 };
                
                // Power: (stage * 0.75 + 1.35) * f * 2.7^d * boss_mult * stage300_mult
                let power = (s * 0.75 + 1.35) * f * 2.7_f64.powf(d_f)
                    * if is_boss { 3.0 } else { 1.0 }
                    * if is_stage_300 { 0.94 } else { 1.0 };
                
                // Crit chance
                let special_chance = (s * 0.0006 + 0.0994 + if is_boss { 0.1 } else { 0.0 }).min(0.25);
                
                // Crit damage
                let special_damage = (s * 0.008 + 1.03).min(2.5);
                
                // Damage reduction
                let dr = if stage >= 200 {
                    let base = 1.0 - (d_f - 2.0).max(0.0) * 0.02 + 0.04;
                    base - if is_boss { 0.05 } else { 0.0 }
                } else {
                    if is_boss { 0.95 } else { 1.0 }
                };
                let actual_dr = 1.0 - dr;
                
                // Evade (starts at stage 100)
                let evade = if stage >= 100 {
                    (d_f - 1.0).max(0.0) * 0.01 + 0.01
                } else { 0.0 };
                
                // Effect chance (starts at stage 300)
                let effect = if stage >= 300 {
                    (d_f - 3.0).max(0.0) * 0.01 + 0.04 + if is_boss { 0.04 } else { 0.0 }
                } else { 0.0 };
                
                // Regen: (stage * 0.1 * f * 1.25^d - 0.08) * boss_mult * stage300_mult
                let regen = (s * 0.1 * f * 1.25_f64.powf(d_f) - 0.08).max(0.0)
                    * if is_boss { 6.0 } else { 1.0 }
                    * if is_stage_300 { 0.97 } else { 1.0 };
                
                // Speed
                let speed = (3.2 - s * 0.004) * if is_boss { 2.45 } else { 1.0 };
                
                (hp, power, regen, special_chance, special_damage, actual_dr, evade, effect, speed)
            }
            HunterType::Knox => {
                let f = Self::knox_scaling(stage);
                
                // HP: (stage * 9 + 7) * f * 3.2^d * boss_mult
                let hp = (s * 9.0 + 7.0) * f * 3.2_f64.powf(d_f)
                    * if is_boss { 120.0 } else { 1.0 };
                
                // Power: (stage * 1.4 + 2.4) * f * 2.7^d * boss_mult
                let power = (s * 1.4 + 2.4) * f * 2.7_f64.powf(d_f)
                    * if is_boss { 4.0 } else { 1.0 };
                
                // Crit chance
                let special_chance = (s * 0.0006 + 0.0994 + if is_boss { 0.1 } else { 0.0 }).min(0.25);
                
                // Crit damage
                let special_damage = (s * 0.008 + 1.032).min(2.5);
                
                // Damage reduction
                let dr = if stage >= 200 {
                    let base = 1.0 - (d_f - 2.0).max(0.0) * 0.02 + 0.04;
                    base - if is_boss { 0.05 } else { 0.0 }
                } else {
                    if is_boss { 0.95 } else { 1.0 }
                };
                let actual_dr = 1.0 - dr;
                
                // Evade
                let evade = (stage / 100) as f64 * 0.01;
                
                // Effect chance (no special formula in CIFI for Knox enemies)
                let effect = 0.0;
                
                // Regen: stage * 0.04 * f * 1.4^d * boss_mult
                let regen = s * 0.04 * f * 1.4_f64.powf(d_f)
                    * if is_boss { 2.0 } else { 1.0 };
                
                // Speed
                let speed = (6.005 - s * 0.005) * if is_boss { 2.85 } else { 1.0 };
                
                (hp, power, regen, special_chance, special_damage, actual_dr, evade, effect, speed)
            }
        }
    }
    
    /// Check if enemy is dead
    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }
    
    /// Apply damage to the enemy
    pub fn take_damage(&mut self, damage: f64) -> f64 {
        let actual = damage * (1.0 - self.damage_reduction);
        self.hp -= actual;
        actual
    }
    
    /// Apply regeneration
    pub fn regen_hp(&mut self) {
        if self.hp < self.max_hp && self.hp > 0.0 {
            self.hp = (self.hp + self.regen).min(self.max_hp);
        }
    }
    
    /// Get attack damage with possible crit - CIFI enrage mechanics
    pub fn get_attack_damage(&self, rng: &mut impl Rng) -> (f64, bool) {
        // At 200+ enrage stacks, damage is tripled and always crits
        let power = if self.enrage_stacks > 200 {
            self.base_power * 3.0
        } else {
            self.base_power
        };
        
        let crit_chance = if self.enrage_stacks > 200 {
            1.0  // Always crit at max enrage
        } else {
            self.special_chance
        };
        
        if rng.gen::<f64>() < crit_chance {
            (power * self.special_damage, true)
        } else {
            (power, false)
        }
    }
    
    /// Add enrage stack (boss only) - CIFI mechanics
    /// Enrage reduces attack speed, doesn't increase power until 200 stacks
    pub fn add_enrage(&mut self) {
        if self.is_boss {
            self.enrage_stacks += 1;
            
            // Speed reduction: speed = base_speed - (stacks * base_speed / 200), min 0.5
            self.speed = (self.base_speed - self.enrage_stacks as f64 * self.base_speed / 200.0).max(0.5);
            
            // Also reduce secondary attack speed
            if self.has_secondary && self.base_speed2 > 0.0 {
                self.speed2 = (self.base_speed2 - self.enrage_stacks as f64 * self.base_speed2 / 200.0).max(0.5);
            }
        }
    }
    
    /// Get current attack speed (accounting for enrage)
    pub fn get_speed(&self) -> f64 {
        self.speed
    }
    
    /// Get current secondary attack speed (accounting for enrage)
    pub fn get_speed2(&self) -> f64 {
        self.speed2
    }
}
