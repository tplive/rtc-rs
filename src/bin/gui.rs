use std::{sync::mpsc, thread, time::Duration};

use eframe::{App, Frame, NativeOptions, egui, run_native};
use egui::{Button, CentralPanel, ColorImage, Context, DragValue, TextureOptions, TopBottomPanel};
use indicatif::ProgressBar;
use rtc::{
    camera::Camera,
    color::Color,
    light::Light,
    material::Material,
    matrix::view_transform,
    plane::Plane,
    render::render_parallel_incremental,
    sphere::Sphere,
    transformation::{scaling, translation},
    tuples::{point, vector},
    util::PI,
    world::World,
};
use rtc_rs::render::render_parallel;

const UPDATE_INTERVAL_MS: u64 = 16;
const PROGRESS_BAR_INTERVAL_MS: u64 = 250;
const MAX_WIDTH: u64 = 16384;
const MAX_HEIGHT: u64 = 16384;

enum RenderMessage {
    Partial { w: usize, h: usize, rgba: Vec<u8> },
    Done { w: usize, h: usize, rgba: Vec<u8> },
}

extern crate rtc_rs as rtc;
struct RayGuiApp {
    image_width: usize,
    image_height: usize,
    is_rendering: bool,
    rx: Option<mpsc::Receiver<RenderMessage>>,
    texture: Option<egui::TextureHandle>,
    status: String,
    show_during_render: bool,
}

impl Default for RayGuiApp {
    fn default() -> Self {
        let image_width = 2560;
        let image_height = 1440;

        Self {
            image_width,
            image_height,
            is_rendering: false,
            rx: None,
            texture: None,
            status: "Idle".to_string(),
            show_during_render: false,
        }
    }
}

impl RayGuiApp {
    fn start_render(&mut self) {
        if self.is_rendering {
            return;
        }

        self.is_rendering = true;
        self.status = "Rendering...".to_string();

        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);

        let w = self.image_width;
        let h = self.image_height;
        let show = self.show_during_render;

        thread::spawn(move || {
            let (camera, world) = build_scene(w, h);

            let bar = if show {
                ProgressBar::hidden()
            } else {

                let bar = ProgressBar::new((w * h) as u64);
                bar.enable_steady_tick(Duration::from_millis(PROGRESS_BAR_INTERVAL_MS));
                bar
            };

            if show {
                let interval = Duration::from_millis(UPDATE_INTERVAL_MS);

                let canvas =
                    render_parallel_incremental(&camera, &world, &bar, false, interval, |c| {
                        let rgba = c.rgba_vec();
                        let _ = tx.send(RenderMessage::Partial {
                            w: c.width,
                            h: c.height,
                            rgba,
                        });
                    });

                let rgba = canvas.rgba_vec();
                let _ = tx.send(RenderMessage::Done {
                    w: canvas.width,
                    h: canvas.height,
                    rgba,
                });
            } else {
                let canvas = render_parallel(&camera, &world, &bar, false);
                let rgba = canvas.to_png();
                let _ = tx.send(RenderMessage::Done {
                    w: canvas.width,
                    h: canvas.height,
                    rgba,
                });
            }
        });
    }
}

impl App for RayGuiApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if let Some(rx) = &self.rx {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    RenderMessage::Partial { w, h, rgba } => {
                        let image = ColorImage::from_rgba_unmultiplied([w, h], &rgba);
                        
                        if let Some(tex) = &mut self.texture {
                            tex.set(image, TextureOptions::LINEAR);
                        } else {
                            self.texture = Some(ctx.load_texture("render", image, TextureOptions::LINEAR));
                        }                
                    }
                    RenderMessage::Done { w, h, rgba } => {
                        let image = ColorImage::from_rgba_unmultiplied([w, h], &rgba);
                        
                        if let Some(tex) = &mut self.texture {
                            tex.set(image, TextureOptions::LINEAR);
                        } else {
                            self.texture = Some(ctx.load_texture("render", image, TextureOptions::LINEAR));
                        }

                        self.status = format!("Done ({}x{})", w, h);
                        self.is_rendering = false;
                        //self.rx = None;
                    }
                }
            }
        }

        TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("rtc-rs ray tracer");
                ui.separator();

                ui.label("W:");
                ui.add(DragValue::new(&mut self.image_width).range(64..=MAX_WIDTH));
                ui.label("H");
                ui.add(DragValue::new(&mut self.image_height).range(64..=MAX_HEIGHT));

                if ui
                    .add_enabled(!self.is_rendering, Button::new("Render"))
                    .clicked()
                {
                    self.start_render();
                }

                ui.checkbox(&mut self.show_during_render, "Display while rendering");
                ui.separator();
                ui.label(&self.status);
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            if let Some(tex) = &self.texture {
                let avail = ui.available_size();
                let mut size = tex.size_vec2();
                size *= (avail.x / size.x).min(avail.y / size.y).min(1.0);
                ui.image((tex.id(), size));
            } else {
                ui.label("Click Render to generate an image.");
            }
        });

        if self.is_rendering {
            ctx.request_repaint();
        }
    }
}

fn build_scene(image_width: usize, image_height: usize) -> (Camera, World) {
    // Floor
    let floor = Plane {
        material: Material::new(Color::random(), None, 0.1, 1.0, 1.0, 1.0),
        ..Default::default()
    };

    // Middle sphere
    let mut middle = Sphere {
        transform: translation(-0.5, 1.0, 0.5),
        ..Default::default()
    };
    middle.material.color = Color::random();
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    // Right sphere
    let mut right = Sphere {
        transform: translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5),
        ..Default::default()
    };
    right.material.color = Color::random();
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    // Left sphere
    let mut left = Sphere {
        transform: translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33),
        ..Default::default()
    };
    left.material.color = Color::random();
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    // World
    let mut world = World::default();

    // Add plane
    world.add_object(floor);

    // Add objects
    world.add_objects(vec![left, middle, right]);

    world.light = vec![Light::point(point(-10.0, 10.0, -10.0), Color::white())];

    let mut camera = Camera::new(image_width, image_height, PI / 3.0);
    camera.transform = view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    );

    (camera, world)
}
fn main() -> eframe::Result<()> {
    let native_options = NativeOptions::default();

    run_native(
        "Raytracer Challenge in Rust",
        native_options,
        Box::new(|_cc| Ok(Box::new(RayGuiApp::default()))),
    )
}
