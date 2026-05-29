use crate::{
    config::Config,
    cs2::{CS2, entity::player::Player},
};

impl CS2 {
    pub fn glow(&self, config: &Config) {
        if !config.misc.glow_enabled {
            return;
        }

        let Some(local_player) = Player::local_player(self) else {
            return;
        };

        let local_team = local_player.team(self);
        let color = config.misc.glow_color;

        let glow_base = self.offsets.glow.m_glow;
        let glow_color = self.offsets.glow.glow_color;
        let glowing = self.offsets.glow.glowing;
        let glow_range = self.offsets.glow.glow_range;
        let glow_type = self.offsets.glow.glow_type;
        let glow_color_override = self.offsets.glow.glow_color_override;

        for player in &self.players {
            if !player.is_valid(self) {
                continue;
            }

            if player.team(self) == local_team && !config.player.show_friendlies {
                continue;
            }

            let pawn = player.pawn;

            self.process.write(pawn + glow_base + glowing, 1u8);
            self.process.write(pawn + glow_base + glow_range, 0xFFFFu32);
            self.process.write(pawn + glow_base + glow_type, 0i32);
            self.process.write(pawn + glow_base + glow_color, [color.r() as f32, color.g() as f32, color.b() as f32]);
            self.process.write(pawn + glow_base + glow_color_override, (255u32 << 24 | (color.r() as u32) << 16 | (color.g() as u32) << 8 | color.b() as u32) as i32);
        }
    }
}
