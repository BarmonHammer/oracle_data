use super::firing_data::StatLerp;

struct Damage {}

struct Explosive {
    value: f64,
    combatant_mult: CombatantMult,
}

struct CombatantMult {
    minor: f64,
    major: f64,
    champion: f64,
    mini_boss: f64,
    boss: f64,
    vehichle: f64,
}
