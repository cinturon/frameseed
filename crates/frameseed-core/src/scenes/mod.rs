mod gradient;
pub use gradient::{GradientScene, GradientParams, palette_from_name, KNOWN_PALETTES};

mod noise_clouds;
pub use noise_clouds::{NoiseCloudsScene, NoiseCloudsParams};

mod conway;
pub use conway::{ConwayScene, ConwayParams};

mod particles;
pub use particles::{ParticlesScene, ParticleParams};

mod flow_field;
pub use flow_field::{FlowFieldScene, FlowFieldParams};

mod sdf_shapes;
pub use sdf_shapes::{SdfShapeScene, SdfShapeParams};

mod mandelbrot;
pub use mandelbrot::{MandelbrotScene, MandelbrotParams};

mod voronoi;
pub use voronoi::{VoronoiScene, VoronoiParams};

mod plasma;
pub use plasma::{PlasmaScene, PlasmaParams};

mod lissajous;
pub use lissajous::{LissajousScene, LissajousParams};

mod sine_wave;
pub use sine_wave::{SineWaveScene, SineWaveParams};

mod starfield;
pub use starfield::{StarfieldScene, StarfieldParams};

mod registry;
pub use registry::{scene_from_config, KNOWN_SCENES};
