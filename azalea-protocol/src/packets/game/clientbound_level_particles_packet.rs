use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;
use azalea_registry::ParticleKind;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundLevelParticlesPacket {
    pub override_limiter: bool,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_dist: f32,
    pub y_dist: f32,
    pub z_dist: f32,
    pub max_speed: f32,
    #[var]
    pub count: u32,
    pub particle: ParticleKind,
}
