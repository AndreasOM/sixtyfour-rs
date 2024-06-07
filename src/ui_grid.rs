use crate::project::GridPos;
use crate::project::GridRect;
use crate::UiGridCell;
use egui::Rect;
use egui::Response;
use egui::Ui;
use egui::Widget;
#[derive(Debug)]
pub struct UiGridOutput {
    selected_grid_rect: Option<GridRect>,
    target_grid_rect: Option<GridRect>,
    response: Response,
    prevent_moving: bool,
}

impl UiGridOutput {
    pub fn selected_grid_rect(&self) -> Option<&GridRect> {
        self.selected_grid_rect.as_ref()
    }
    pub fn target_grid_rect(&self) -> Option<&GridRect> {
        self.target_grid_rect.as_ref()
    }
    pub fn target_grid_pos(&self) -> Option<&GridPos> {
        self.target_grid_rect.as_ref().map(|gr| gr.top_left())
    }
    pub fn prevent_moving(&self) -> bool {
        self.prevent_moving
    }
}

//#[derive(Debug)]
pub struct UiGrid {
    id: egui::Id,
    cell_width: f32,
    cell_height: f32,
    cell_size: egui::Vec2,
    width: u16,
    height: u16,
    cells: Vec<Option<UiGridCell>>, // :HACK: this should be sparse
    selected_rect: Option<GridRect>,
    highlighted_cells: Vec<GridPos>,
    target_rect: Option<GridRect>,
    zoom: f32,
    target_zoom: f32,
}

#[derive(Debug, Default, Clone)]
pub enum State {
    Selecting {
        start: egui::Pos2,
    },
    #[default]
    Normal,
}

impl State {
    pub fn prevent_moving(&self) -> bool {
        match self {
            Self::Selecting { .. } => true,
            _ => false,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct UiGridTemp {
    state: State,
}

impl UiGridTemp {
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
}

impl Default for UiGrid {
    fn default() -> Self {
        let width = 32u16;
        let height = 32u16;
        let mut cells = Vec::with_capacity((width * height).into());
        cells.resize_with((width * height).into(), Default::default);

        Self {
            id: egui::Id::NULL,
            cell_width: 128.0,
            cell_height: 32.0,
            cell_size: egui::Vec2::new(128.0, 32.0),
            width,
            height,
            cells,
            //selected_cell: None,
            selected_rect: None,
            highlighted_cells: Vec::default(),
            target_rect: None,
            zoom: 1.0,
            target_zoom: 1.0,
        }
    }
}

impl UiGrid {
    /*
    pub fn target_grid_pos(&self) -> Option<&GridPos> {
        self.target_grid_pos.as_ref()
    }
    */
    pub fn set_id(&mut self, id: egui::Id) {
        self.id = id;
    }
    pub fn set_zoom(&mut self, zoom: f32) {
        self.target_zoom = zoom;
    }
    pub fn set_target_rect(&mut self, target_rect: Option<&GridRect>) {
        self.target_rect = target_rect.cloned();
    }
    pub fn add_cell(&mut self, x: u16, y: u16, content: UiGridCell) {
        let offset = (y * self.width + x) as usize;
        if offset > self.cells.capacity() {
            return;
        }
        self.cells[offset] = Some(content);
    }

    /*
    pub fn select_cell(&mut self, pos: Option<&GridPos>) {
        self.selected_cell = pos.cloned();
    }
    */

    pub fn select_rect(&mut self, rect: Option<&GridRect>) {
        self.selected_rect = rect.cloned();
    }

    pub fn highlight_cell(&mut self, pos: &GridPos) {
        self.highlighted_cells.push(pos.to_owned());
    }

    fn paint_highlight_cell(
        &self,
        ui: &mut Ui,
        stroke: Option<&egui::Stroke>,
        fill: Option<&egui::Color32>,
        pos: &GridPos,
    ) {
        let pos = ui.min_rect().min
            + self.cell_size
                * self.zoom
                * egui::Vec2::new(pos.x() as f32 + 0.5, pos.y() as f32 + 0.5);
        let rect = Rect::from_center_size(pos, self.cell_size * self.zoom);
        let rounding = 0.125 * self.cell_size.y * self.zoom;
        if let Some(fill) = fill {
            ui.painter().rect_filled(rect, rounding, *fill);
        }
        if let Some(stroke) = stroke {
            let mut stroke = stroke.clone();
            stroke.width *= self.zoom;
            ui.painter().rect_stroke(rect, rounding, stroke);
        }
    }
    fn paint_highlight_rect(
        &self,
        ui: &mut Ui,
        stroke: Option<&egui::Stroke>,
        fill: Option<&egui::Color32>,
        rect: &GridRect,
    ) {
        let min = ui.min_rect().min
            + self.cell_size
                * self.zoom
                * egui::Vec2::new(
                    rect.top_left().x() as f32 - 0.0,
                    rect.top_left().y() as f32 - 0.0,
                );
        let max = ui.min_rect().min
            + self.cell_size
                * self.zoom
                * egui::Vec2::new(
                    rect.bottom_right().x() as f32 + 1.0,
                    rect.bottom_right().y() as f32 + 1.0,
                );
        let rect = Rect::from_min_max(min, max);
        let rounding = 0.125 * self.cell_size.y * self.zoom;
        if let Some(fill) = fill {
            ui.painter().rect_filled(rect, rounding, *fill);
        }
        if let Some(stroke) = stroke {
            let mut stroke = stroke.clone();
            stroke.width *= self.zoom;
            ui.painter().rect_stroke(rect, rounding, stroke);
        }
    }

    fn screen_pos_to_grid_pos(&self, ul: &egui::Pos2, screen_pos: &egui::Pos2) -> GridPos {
        let p = *screen_pos - *ul;
        let p = p / (self.cell_size * self.zoom);
        let p = p.floor();

        GridPos::new(p.x as u16, p.y as u16)
    }
    pub fn show(mut self, ui: &mut Ui) -> UiGridOutput {
        let mut temp = ui.data(|d| {
            if let Some(t) = d.get_temp::<UiGridTemp>(self.id) {
                t
            } else {
                UiGridTemp::default()
            }
        });
        self.zoom =
            ui.ctx()
                .animate_value_with_time(egui::Id::new("UiGridZoom"), self.target_zoom, 0.1);
        //self.zoom = self.target_zoom;
        let desired_size = egui::vec2(
            self.cell_width * self.width as f32 * self.zoom,
            self.cell_height * self.height as f32 * self.zoom,
        );
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        //let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());
        let mut selected_grid_rect = None;
        if ui.is_rect_visible(rect) {
            let cell_size = egui::Vec2::new(self.cell_width, self.cell_height);

            // paint grid
            let stroke = egui::Stroke::new(0.25 * self.zoom, egui::Color32::from_rgb(50, 50, 50));
            //let stroke = egui::Stroke::new(2.25*self.zoom, egui::Color32::from_rgb(250, 250, 150));
            // let visuals = ui.style().interact_selectable(&response, true);

            let vis_r = response.interact_rect; //.translate( ui.min_rect().min.to_vec2() );
            let c = ((vis_r.width() / (self.cell_width * self.zoom)).ceil()) as usize;
            let c = c + 1;
            let ul = ui.min_rect().min; // upper left of "window"
            let lr = ui.min_rect().max; // lower right of "window"

            let p = vis_r.min - ul;
            let p = p / (egui::Vec2::new(self.cell_width, self.cell_height) * self.zoom);
            let p = p.floor();
            let p = p * (egui::Vec2::new(self.cell_width, self.cell_height) * self.zoom);
            let mut p = ul + p;

            // vertical lines
            for _ in 0..c {
                ui.painter().vline(
                    p.x,
                    egui::Rangef::new(ul.y, lr.y),
                    stroke, //visuals.bg_stroke,
                );
                p.x += self.cell_width * self.zoom;
            }

            let c = ((vis_r.height() / self.cell_height / self.zoom).ceil()) as usize;
            let c = c + 1;
            for _ in 0..c {
                ui.painter().hline(
                    egui::Rangef::new(ul.x, lr.x),
                    p.y,
                    stroke, //visuals.bg_stroke,
                );
                p.y += self.cell_height * self.zoom;
            }

            // paint highlights

            if let Some(rect) = &self.selected_rect {
                let stroke = egui::Stroke::new(9.0, egui::Color32::from_rgb(175, 150, 50));
                let fill = egui::Color32::from_rgba_unmultiplied(175, 150, 50, 16);
                self.paint_highlight_rect(ui, Some(&stroke), Some(&fill), &rect);
                let stroke = egui::Stroke::new(5.0, egui::Color32::from_rgb(75, 50, 50));
                self.paint_highlight_cell(ui, Some(&stroke), None, rect.top_left());
            }
            if let Some(rect) = &self.target_rect {
                let stroke = egui::Stroke::new(9.0, egui::Color32::from_rgb(95, 105, 55));
                let fill = egui::Color32::from_rgba_unmultiplied(95, 105, 55, 16);
                self.paint_highlight_rect(ui, Some(&stroke), Some(&fill), &rect);
                let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(75, 100, 50));
                self.paint_highlight_cell(ui, Some(&stroke), None, rect.top_left());
            }
            let cells = core::mem::take(&mut self.cells);

            for (idx, content) in cells.into_iter().enumerate() {
                let y = idx / self.width as usize;
                let x = idx % self.width as usize;
                let cell_pos = egui::Pos2::new(
                    self.cell_width * self.zoom * ((x as f32) + 0.5),
                    self.cell_height * self.zoom * ((y as f32) + 0.5),
                );
                let cell_pos = ui.min_rect().min + cell_pos.to_vec2();
                let cell_rect = egui::Rect::from_center_size(cell_pos, cell_size * self.zoom);
                let cell_rect = cell_rect.shrink(1.0);

                if let Some(mut content) = content {
                    //let r = ui.put(cell_rect, UiGridCell::new(content.to_string()));
                    //let content = Box::new( egui::Label::new("fii") );
                    content.set_zoom(self.zoom);
                    let r = ui.put(cell_rect, content);

                    if r.clicked() {
                        let clicked_gp = GridPos::new(x as u16, y as u16);
                        if ui.ctx().input(|i| i.modifiers.shift) {
                            if let Some(selected_rect) = &self.selected_rect {
                                // selected_grid_rect == None!
                                assert_eq!(selected_grid_rect, None);
                                let r = selected_rect.union_with_pos(&clicked_gp);
                                eprintln!("Shift Clicked {clicked_gp:?} -> {r:?}");
                                selected_grid_rect = Some(r);
                            } else {
                                // :TODO:
                            }
                        } else {
                            let mut r = GridRect::default();
                            r.set_top_left(&clicked_gp);
                            r.set_size(GridPos::zero());
                            selected_grid_rect = Some(r);
                            //selected_grid_pos = Some(GridPos::new(x as u16, y as u16));
                        }
                    }
                    if r.secondary_clicked() {
                        if let Some(cp) = r.interact_pointer_pos() {
                            let ghp = self.screen_pos_to_grid_pos(&ui.min_rect().min, &cp);
                            //let stroke =
                            //    egui::Stroke::new(2.25, egui::Color32::from_rgb(250, 150, 100));

                            //self.paint_highlight_cell(ui, &stroke, &ghp);
                            let mut r = GridRect::default();
                            r.set_top_left(&ghp);
                            r.set_size(GridPos::zero());
                            self.target_rect = Some(r);
                        }
                    }
                } else {
                }
            }
        }

        if let Some(hp) = response.hover_pos() {
            let ghp = self.screen_pos_to_grid_pos(&ui.min_rect().min, &hp);
            let stroke = egui::Stroke::new(2.25, egui::Color32::from_rgb(50, 150, 200));
            let fill = egui::Color32::from_rgba_unmultiplied(25, 75, 100, 16);
            self.paint_highlight_cell(ui, Some(&stroke), Some(&fill), &ghp);
        }

        // response.paint_debug_info();

        /*
        if response.is_pointer_button_down_on() {
            eprintln!("Button down!");
        }
        */

        let mut rect = None;
        if response.contains_pointer() {
            ui.ctx().input_mut(|i| {
                //eprintln!("{:#?}", i.pointer );
                match temp.state() {
                    State::Normal => {
                        if i.pointer.button_pressed(egui::PointerButton::Primary) {
                            if let Some(p) = i.pointer.interact_pos() {
                                eprintln!("Start selection");
                                temp.set_state(State::Selecting { start: p.clone() });
                            }
                        }
                    }
                    State::Selecting { start } => {
                        if let Some(current) = i.pointer.interact_pos() {
                            rect = Some(egui::Rect::from_two_pos(*start, current));
                        }
                        if i.pointer.button_released(egui::PointerButton::Primary) {
                            eprintln!("End selection");
                            temp.set_state(State::Normal);
                        }
                    }
                    _ => {
                        eprintln!("State: {:?}", temp.state());
                    }
                }
            });
        }

        if let Some(rect) = rect {
            let rounding = 0.0;
            let stroke = egui::Stroke::new(2.25, egui::Color32::from_rgb(250, 150, 100));
            ui.painter().rect_stroke(rect, rounding, stroke);
        }

        if response.secondary_clicked() {
            if let Some(cp) = response.interact_pointer_pos() {
                let ghp = self.screen_pos_to_grid_pos(&ui.min_rect().min, &cp);
                //let stroke = egui::Stroke::new(2.25, egui::Color32::from_rgb(250, 150, 100));
                //self.paint_highlight_cell(ui, &stroke, &ghp);
                let mut r = GridRect::default();
                r.set_top_left(&ghp);
                r.set_size(GridPos::zero());
                self.target_rect = Some(r);
            }
        }

        match (self.selected_rect, &mut self.target_rect) {
            (Some(sgr), Some(tr)) => {
                tr.set_size(&sgr.size());
            }
            _ => {}
        };

        let temp2 = temp.clone();
        ui.data_mut(|d| {
            d.insert_temp(self.id, temp2);
        });

        UiGridOutput {
            selected_grid_rect,
            response,
            target_grid_rect: self.target_rect,
            prevent_moving: temp.state().prevent_moving(),
        }
    }
}

impl Widget for UiGrid {
    fn ui(self, ui: &mut Ui) -> Response {
        // todo!();
        self.show(ui).response
    }
}
