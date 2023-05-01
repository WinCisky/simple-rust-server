use crate::handler::MOVE;

#[derive(Clone)]
pub struct Entity {
    pub pos_x: f32,
    pub pos_y: f32,
    pub level: u8,
    pub id: u32,
    pub user_id: u32,
    pub health: u8,
}

impl Entity {
    pub fn format_message_packet_movement(&self) -> [u8; 13] {
        let mut buf = [MOVE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, self.level, self.health];
        let bytes = self.pos_x.to_le_bytes();
        buf[1..5].copy_from_slice(&bytes);
        let bytes = self.pos_y.to_le_bytes();
        buf[5..9].copy_from_slice(&bytes);
        let bytes = self.user_id.to_le_bytes();
        buf[9..13].copy_from_slice(&bytes);
        buf
    }
}