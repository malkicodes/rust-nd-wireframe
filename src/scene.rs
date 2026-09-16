use std::{fs, io};

use macroquad::color::Color;
use nalgebra::{DVector, Vector2};
use serde::Deserialize;

#[derive(Debug)]
pub struct Scene {
    pub polytopes_folder: String,
    pub polytope_path: String,
    pub resolution: u32,
    pub frame_count: i32,
    pub facet_expansion: f32,
    pub facet_expansion_rank: usize,
    pub min_dimension: usize,
    pub dimension: usize,
    pub vertices: Vec<DVector<f32>>,
    pub edges: Vec<usize>,
    pub edge_colors: Vec<Color>,
    pub resolution_vector: Vector2<f32>,
}

impl Scene {
    pub fn setup(args: &[String]) -> Self {
        let given_polytope_path = args.get(1).cloned();

        // if you can read setup.toml
        if let Ok(bytes) = fs::read("./setup.toml") {
            let scene_config: SceneConfig =
                toml::from_slice(&bytes).expect("error reading setup.toml");

            // convert SceneConfig -> Scene and return
            let mut scene: Scene = scene_config.into();

            if let Some(polytope_path) = given_polytope_path {
                scene.polytope_path = polytope_path;
            }

            return scene;
        }

        let setup_file_contents = match fs::read_to_string("./setup.txt") {
            Ok(v) => v,
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => panic!("no setup.txt file!!!!"),
                _ => panic!("could not read setup.txt: {}", err),
            },
        };
        let lines: Vec<&str> = setup_file_contents.lines().collect();

        Scene {
            polytopes_folder: lines[0].to_string(),
            polytope_path: given_polytope_path.unwrap_or_else(|| lines[1].to_string()),
            resolution: lines[2].parse().unwrap(),
            frame_count: lines[3].parse().unwrap(),
            min_dimension: lines[4].parse().unwrap(),
            facet_expansion: lines[5].parse().unwrap(),
            facet_expansion_rank: lines[6].parse::<isize>().unwrap() as usize, // converts negative values to super high (integer underflow) ones. necessary for relative to rank values
            dimension: 0,
            vertices: vec![],
            edges: vec![],
            edge_colors: vec![],
            resolution_vector: Vector2::new(lines[2].parse().unwrap(), lines[2].parse().unwrap()),
        }
    }

    pub fn clear_polytope(&mut self) {
        self.vertices.clear();
        self.edges.clear();
    }
}

#[derive(Debug, Deserialize)]
struct SceneConfig {
    polytopes_folder: String,
    polytope_path: String,

    min_dimension: usize,

    #[serde(default)]
    animation: AnimationConfig,
    #[serde(default)]
    facet_expansion: FacetExpansionConfig,
}

impl From<SceneConfig> for Scene {
    fn from(value: SceneConfig) -> Self {
        Scene {
            polytopes_folder: value.polytopes_folder,
            polytope_path: value.polytope_path,
            resolution: value.animation.resolution,
            frame_count: value.animation.frame_count,
            min_dimension: value.min_dimension,
            facet_expansion: value.facet_expansion.facet_expansion,
            facet_expansion_rank: value.facet_expansion.facet_expansion_rank as usize, // converts negative values to super high (integer underflow) ones. necessary for relative to rank values
            dimension: 0,
            vertices: vec![],
            edges: vec![],
            edge_colors: vec![],
            resolution_vector: Vector2::new(
                value.animation.resolution as f32,
                value.animation.resolution as f32,
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
struct AnimationConfig {
    resolution: u32,
    frame_count: i32,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            resolution: 1200,
            frame_count: 80,
        }
    }
}

#[derive(Debug, Deserialize)]
struct FacetExpansionConfig {
    #[serde(default, alias = "amount")]
    facet_expansion: f32,
    #[serde(
        default = "FacetExpansionConfig::default_facet_expansion_rank",
        alias = "rank"
    )]
    facet_expansion_rank: isize,
}

impl FacetExpansionConfig {
    const fn default_facet_expansion_rank() -> isize {
        -1
    }
}

impl Default for FacetExpansionConfig {
    fn default() -> Self {
        Self {
            facet_expansion: 0.0,
            facet_expansion_rank: -1,
        }
    }
}
