mod gradient;
#[allow(unused_imports)]
pub use gradient::{
    GradientDirection, GradientParams, GradientScene, KNOWN_PALETTES, palette_from_name,
};

mod noise_clouds;
pub use noise_clouds::{NoiseCloudsParams, NoiseCloudsScene};

mod conway;
pub use conway::{ConwayParams, ConwayScene};

mod particles;
pub use particles::{ParticleParams, ParticlesScene};

mod flow_field;
pub use flow_field::{FlowFieldParams, FlowFieldScene};

mod sdf_shapes;
pub use sdf_shapes::{SdfShapeParams, SdfShapeScene};

mod mandelbrot;
pub use mandelbrot::{MandelbrotParams, MandelbrotScene};

mod voronoi;
pub use voronoi::{VoronoiParams, VoronoiScene};

mod plasma;
pub use plasma::{PlasmaParams, PlasmaScene};

mod lissajous;
pub use lissajous::{LissajousParams, LissajousScene};

mod sine_wave;
pub use sine_wave::{SineWaveParams, SineWaveScene};

mod starfield;
pub use starfield::{StarfieldParams, StarfieldScene};

mod tunnel;
pub use tunnel::{TunnelParams, TunnelScene};

mod kaleidoscope;
pub use kaleidoscope::{KaleidoscopeParams, KaleidoscopeScene};

mod metaballs;
pub use metaballs::{MetaballsParams, MetaballsScene};

mod oscilloscope;
pub use oscilloscope::{OscilloscopeParams, OscilloscopeScene};

mod blend;
pub use blend::{BlendParams, BlendScene, ConfigScene};

mod registry;
pub use registry::{KNOWN_SCENES, scene_from_config};
