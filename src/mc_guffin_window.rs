use crate::command_queue::COMMAND_QUEUE;
use crate::project::PropertyValue;
use crate::state::State;
use crate::window::Window;
use crate::Command;
use color_eyre::Result;
use egui::Key;
use egui::Rect;

#[derive(Default)]
pub struct McGuffinWindow {
    is_open: bool,
    was_fullscreen: bool,

    previous_rect: Option<Rect>,
}

impl core::fmt::Debug for McGuffinWindow {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct McGuffinWindowSave {
    #[serde(default)]
    is_open: bool,

    #[serde(default)]
    previous_rect: Option<Rect>,
}

impl From<&McGuffinWindow> for McGuffinWindowSave {
    fn from(mw: &McGuffinWindow) -> Self {
        Self {
            is_open: mw.is_open,
            previous_rect: mw.previous_rect.clone(),
        }
    }
}

impl Window for McGuffinWindow {
    fn name(&self) -> &str {
        "McGuffin"
    }
    fn is_open(&self) -> bool {
        self.is_open
    }
    fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
    fn serialize(&self) -> String {
        let save: McGuffinWindowSave = self.into();

        ron::ser::to_string(&save).unwrap_or_default()
    }
    fn deserialize(&mut self, data: &str) {
        let mut save: McGuffinWindowSave = ron::from_str(&data).unwrap_or_default();

        self.is_open = save.is_open;
        self.previous_rect = save.previous_rect.take();
    }

    fn update(&mut self, ctx: &egui::Context, state: &mut State) {
        let mut is_open = self.is_open;
        // target:  [[0.0 23.8] - [257.0 189.5]]
        let mut w = egui::Window::new("McGuffin")
            .frame(
                egui::Frame::window( &egui::Style::default() )
                    .inner_margin( egui::Margin::ZERO )
                    .outer_margin( egui::Margin::ZERO )
            )
            .resizable(!state.mc_guffin_is_fullscreen)
            .movable(!state.mc_guffin_is_fullscreen)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            .title_bar(!state.mc_guffin_is_fullscreen)
            .open(&mut is_open)
            //.resize(|r| r.auto_expand_width(true))
            ;

        if state.mc_guffin_is_fullscreen {
            if !self.was_fullscreen {
                // just became fullscreen
                eprintln!("! {:#?}", self.previous_rect);
                self.was_fullscreen = true;
            }
            w = w.fixed_pos(egui::Pos2::ZERO);
            w = w.fixed_size(egui::Vec2::new(16000.0, 9000.0));
        } else {
            if self.was_fullscreen {
                // just ended fullscreen
                if let Some(rect) = self.previous_rect {
                    // w = w.fixed_rect( rect );
                    w = w.current_pos(rect.min);
                    let mut s = rect.size().to_owned();
                    s.x -= 1.0; // no idea?
                    w = w.fixed_size(s);
                }
                self.was_fullscreen = false;
            }
        }

        if let Some(ir) = w.show(ctx, |ui| {
            if state.mc_guffin_is_fullscreen {
                //ui.allocate_space(ui.available_size());
            }
            if state.mc_guffin_is_fullscreen && ui.input(|i| i.key_pressed(Key::Escape)) {
                let _ = COMMAND_QUEUE.send(Command::LeaveFullscreen);
            }
            self.mc_guffin_painting(ui, state);
        }) {
            if !state.mc_guffin_is_fullscreen {
                //eprintln!("! {:#?} == [[0.0 23.8] - [257.0 189.5]]", ir.response.rect);
                self.previous_rect = Some(ir.response.rect);
                //self.previous_rect = Some(ir.response.interact_rect);
            }
        }
        self.is_open = is_open;
    }
}

impl McGuffinWindow {
    fn mc_guffin_painting(&mut self, ui: &mut egui::Ui, state: &mut State) {
        let s = ui.available_size();

        let mut wanted_size = egui::Vec2::new(256.0, 144.0);
        let sx = s.x / wanted_size.x;
        let sy = s.y / wanted_size.y;

        let scale = sx.min(sy).max(1.0);
        wanted_size *= scale;

        let (rect, sense) = ui.allocate_at_least(wanted_size, egui::Sense::click_and_drag());
        /*
        eprintln!("Rect {rect:?}");
        if !state.mc_guffin_is_fullscreen {
            self.previous_rect = Some( rect );
        }
        */
        if let Some(mc_guffin) = state.mc_guffin_cloned() {
            let callback = egui::PaintCallback {
                rect,
                callback: std::sync::Arc::new(eframe::egui_glow::CallbackFn::new(
                    move |_info, painter| {
                        mc_guffin.lock().paint(painter.gl());
                    },
                )),
            };
            ui.painter().add(callback);
        }
        if let Some(click_pos) = sense.interact_pointer_pos() {
            let rs = rect.max - rect.min;
            let np = ((click_pos - rect.min) / rs) * egui::Vec2::new(2.0, -2.0)
                + egui::Vec2::new(-1.0, 1.0);

            state.project.with_property_manager_mut(|pm| {
                if let Some(p) = pm.get_mut("fMouseClick") {
                    match p.value_mut() {
                        PropertyValue::Vec2F32 { ref mut values } => {
                            values[0] = np.x;
                            values[1] = np.y;
                        }
                        _ => {}
                    }
                }
            });
        }
        if let Some(click_pos) = sense.hover_pos() {
            let rs = rect.max - rect.min;
            let np = ((click_pos - rect.min) / rs) * egui::Vec2::new(2.0, -2.0)
                + egui::Vec2::new(-1.0, 1.0);

            state.project.with_property_manager_mut(|pm| {
                if let Some(p) = pm.get_mut("fMouseHover") {
                    match p.value_mut() {
                        PropertyValue::Vec2F32 { ref mut values } => {
                            values[0] = np.x;
                            values[1] = np.y;
                        }
                        _ => {}
                    }
                }
            });
        }
    }
}
