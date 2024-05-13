use crate::project::Property;
use crate::project::PropertyConfig;
use crate::project::PropertyValue;
use crate::PropertyUiValue;
use egui::Color32;
use egui::ColorImage;
use egui::ImageData;
use egui::RichText;
use egui::TextureHandle;
use egui::TextureOptions;
use egui::WidgetText;
use std::collections::HashMap;

struct Preview {
    texture_handle: TextureHandle,
    color_image: ColorImage,
}
impl core::fmt::Debug for Preview {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        Ok(())
    }
}

impl Preview {
    pub fn new(ctx: &egui::Context, width: usize, height: usize, name: &str) -> Self {
        let color_image = ColorImage::new([width, height], Color32::PLACEHOLDER);
        let options = TextureOptions::default();
        let texture_handle = ctx.load_texture(name, color_image.clone(), options);

        Self {
            texture_handle,
            color_image,
        }
    }
    fn update(&mut self, values: &[f32; 3 * 4]) {
        let img = &mut self.color_image;
        let w = img.size[0];
        let h = img.size[1];
        let xs = 1.0 / (w as f32);

        for y in 0..h {
            for x in 0..w {
                let ofs = y * w + x;
                let pixel = &mut img.pixels[ofs];
                // a + b*cos( 6.28318*(c*i+d) );
                // ra ga ba
                // rb gb bb
                // rc gc bc
                // rd gd bd

                let i = x as f32 * xs;
                let r = values[0 * 3 + 0]
                    + values[1 * 3 + 0]
                        * (6.28318 * (values[2 * 3 + 0] * i + values[3 * 3 + 0])).cos();
                let g = values[0 * 3 + 1]
                    + values[1 * 3 + 1]
                        * (6.28318 * (values[2 * 3 + 1] * i + values[3 * 3 + 1])).cos();
                let b = values[0 * 3 + 2]
                    + values[1 * 3 + 2]
                        * (6.28318 * (values[2 * 3 + 2] * i + values[3 * 3 + 2])).cos();

                //                *pixel = Color32::GOLD;
                *pixel = Color32::from_rgb(
                    (255.0 * r).floor() as u8,
                    (255.0 * g).floor() as u8,
                    (255.0 * b).floor() as u8,
                );
            }
        }
    }
}
#[derive(Default)]
pub struct PropertyUiValueVec3F32Size4 {
    previews: HashMap<String, Preview>,
    // texture_handle: Option<TextureHandle>,
    // image_data: Option<ColorImage>,
}

impl core::fmt::Debug for PropertyUiValueVec3F32Size4 {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        Ok(())
    }
}
impl PropertyUiValueVec3F32Size4 {}

impl PropertyUiValue for PropertyUiValueVec3F32Size4 {
    fn label(&self, name: &str, property: &mut Property) -> Option<WidgetText> {
        match (&mut property.value, &mut property.config) {
            (
                PropertyValue::Vec3F32Size4 { values },
                PropertyConfig::F32 {
                    min_value: _,
                    max_value: _,
                    step_size: _,
                },
            ) => Some(
                format!(
                    "{name} {:.3}, {:.3}, {:.3}",
                    values[0], values[1], values[2]
                )
                .into(),
            ),
            (PropertyValue::Vec3F32Size4 { values }, PropertyConfig::ColorPal {}) => Some(
                format!(
                    "{name} {:.3}, {:.3}, {:.3}",
                    values[0], values[1], values[2]
                )
                .into(),
            ),
            _ => None,
        }
    }
    fn update(&mut self, ui: &mut egui::Ui, name: &str, property: &mut Property) -> bool {
        match (&mut property.value, &mut property.config) {
            (
                PropertyValue::Vec3F32Size4 { values },
                PropertyConfig::F32 {
                    min_value,
                    max_value,
                    step_size,
                },
            ) => {
                ui.vertical(|ui| {
                    //ui.label(name);
                    //egui::widgets::color_picker::color_edit_button_rgb( ui, &mut *values);

                    for i in 0..12 {
                        ui.add(
                            egui::Slider::new(&mut (*values)[i], *min_value..=*max_value)
                                .step_by(*step_size as f64)
                                .text(format!("{i}")),
                        );
                    }
                });
                true
            }
            (PropertyValue::Vec3F32Size4 { values }, PropertyConfig::ColorPal {}) => {
                if let Some(preview) = self.previews.get_mut(name) {
                    preview.update(values);
                    //Self::update_image( &mut preview.color_image, values);
                    let options = TextureOptions::default();
                    preview
                        .texture_handle
                        .set(preview.color_image.clone(), options);
                } else {
                    let preview = Preview::new(ui.ctx(), 64, 64, name);
                    self.previews.insert(name.to_owned(), preview);
                }
                ui.vertical(|ui| {
                    //ui.label(name);
                    //egui::widgets::color_picker::color_edit_button_rgb( ui, &mut *values);

                    egui::Grid::new(name).show(ui, |ui| {
                        ui.label("TL");
                        ui.vertical(|ui| {
                            for i in 6..=8 {
                                // c - rgb
                                ui.add(
                                    egui::Slider::new(&mut (*values)[i], 0.0..=2.0)
                                        .step_by(0.1 as f64)
                                        .text(format!("{i}")),
                                );
                            }
                        });
                        ui.label("TR");
                        ui.end_row();
                        ui.horizontal(|ui| {
                            for i in 0..=2 {
                                // c - rgb
                                ui.add(
                                    egui::Slider::new(&mut (*values)[i], 0.0..=2.0)
                                        .vertical()
                                        .step_by(0.1 as f64)
                                        .text(format!("{i}")),
                                );
                            }
                        });
                        //ui.label("MIDDLE");
                        // -----
                        if let Some(preview) = self.previews.get_mut(name) {
                            let sized_image = egui::load::SizedTexture::new(
                                preview.texture_handle.id(),
                                egui::vec2(1.5 * 128.0, 128.0),
                            );
                            let image = egui::Image::from_texture(sized_image);
                            ui.add(image);
                        } else {
                            // unimplemented!();
                            ui.label("unimplemented");
                        }

                        // -----
                        ui.horizontal(|ui| {
                            for i in 3..=5 {
                                // c - rgb
                                ui.add(
                                    egui::Slider::new(&mut (*values)[i], 0.0..=2.0)
                                        .vertical()
                                        .step_by(0.1 as f64)
                                        .text(format!("{i}")),
                                );
                            }
                        });
                        ui.end_row();
                        ui.separator();
                        ui.separator();
                        ui.separator();
                        ui.end_row();
                        ui.label("BL");
                        ui.vertical(|ui| {
                            for i in 9..=11 {
                                // c - rgb
                                ui.add(
                                    egui::Slider::new(&mut (*values)[i], 0.0..=2.0)
                                        .step_by(0.1 as f64)
                                        .text(format!("{i}")),
                                );
                            }
                        });
                        ui.label("BR");
                    });
                });
                true
            }
            _ => false,
        }
    }
}
