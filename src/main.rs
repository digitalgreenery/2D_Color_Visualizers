use bevy::prelude::*;
use prismatic_color::Color as P_Color;

mod hue_wheel;
mod color_peaks;
mod gradients;

#[derive(Component, Clone)]
pub struct VisualizerComponent;

#[derive(Clone)]
pub enum VisualizerScene{
    HueWheel,
    ColorPeaks,
    Gradients,
}



trait ColorVisualizer{
    fn spawn(
        &self,
        window: Query<&Window>,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
        images: &mut ResMut<Assets<Image>>,
        color_sets: Vec<Vec<P_Color>>,
    );
    fn despawn(
        commands: &mut Commands,
        query: Query<Entity, With<VisualizerComponent>>
    );
    fn generate_colors(
        &self,
        //Need to add color augmentation
    ) -> Vec<Vec<P_Color>>;
}

impl ColorVisualizer for VisualizerScene {
    fn spawn(
        &self,
        window: Query<&Window>,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
        images: &mut ResMut<Assets<Image>>,
        color_sets: Vec<Vec<P_Color>>,
    ) {
        match self {
            VisualizerScene::HueWheel => hue_wheel::spawn(window, commands, materials, meshes, color_sets),
            VisualizerScene::ColorPeaks => color_peaks::spawn(window, commands, materials, meshes, color_sets),
            VisualizerScene::Gradients => gradients::spawn(window, commands, materials, meshes, images, color_sets),
        }
    }

    fn despawn(

        commands: &mut Commands,
        query: Query<Entity, With<VisualizerComponent>>
    ) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }

    fn generate_colors(
        &self,
        //Need to add color augmentation
    ) -> Vec<Vec<P_Color>> {
        match self {
            VisualizerScene::HueWheel => hue_wheel::generate_hues(),
            VisualizerScene::ColorPeaks => color_peaks::generate_colors(),
            VisualizerScene::Gradients => gradients::generate_colors(),
        }
    }
}

#[derive(Resource)]
struct SceneConfig {
    pos: usize,
    scenes: Vec<VisualizerScene>,
}

impl SceneConfig {
    fn new() -> SceneConfig {
        SceneConfig { 
            pos: 0,
            scenes: vec![
                VisualizerScene::HueWheel,
                VisualizerScene::ColorPeaks,
                VisualizerScene::Gradients,
            ],
        }
    }
    fn advance(&mut self) {
        self.pos = 
            if self.pos + 1 < self.scenes.len() { self.pos + 1} else {0};
    }
    fn spawn_scene(
        &self,
        windows: Query<&Window>,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        images: &mut ResMut<Assets<Image>>,
    ) {
        let scene = self.scenes.get(self.pos).expect("Scene out of range");
        let colors = scene.generate_colors();
        scene.spawn(windows, commands, materials, meshes, images, colors);
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
    .add_systems(Startup, setup);
    app.add_systems(Update, toggle_visualizers);
    app.insert_resource(SceneConfig::new());
    app.run();
}

fn setup(
    windows: Query<&Window>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut scene_config: ResMut<SceneConfig>,
) {
    commands.spawn(Camera2dBundle::default());

    *scene_config = SceneConfig::new();
    scene_config.spawn_scene(windows, &mut commands, &mut meshes, &mut materials, &mut images);
                            
    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn(
        TextBundle::from_section("Press space to toggle visualizations", TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
    );
}

fn toggle_visualizers(
    window: Query<&Window>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    visualizer_components: Query<Entity, With<VisualizerComponent>>,
    mut scene_config: ResMut<SceneConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {

    if keyboard.just_pressed(KeyCode::Space) {
        VisualizerScene::despawn(&mut commands, visualizer_components);
        scene_config.advance();
        scene_config.spawn_scene(window, &mut commands, &mut meshes, &mut materials, &mut images);
    }
}
 