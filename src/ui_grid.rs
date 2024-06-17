use crate::project::GridPos;
use crate::project::GridRect;
use crate::UiGridCell;
use egui::Key;
use egui::Rect;
use egui::Response;
use egui::Ui;
use egui::Widget;
#[derive(Debug)]
pub enum UiGridAction {
    Copy {
        source_rect: GridRect,
        target_pos: GridPos,
    },
    Move {
        source_rect: GridRect,
        target_pos: GridPos,
    },
}

#[derive(Debug)]
pub struct UiGridOutput {
    selected_grid_rect: Option<GridRect>,
    target_grid_rect: Option<GridRect>,
    response: Response,
    prevent_moving: bool,
    action: Option<UiGridAction>,
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
    pub fn action(&self) -> Option<&UiGridAction> {
        self.action.as_ref()
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
    used_cells: Vec<Option<()>>,
    selected_rect: Option<GridRect>,
    highlighted_cells: Vec<GridPos>,
    target_rect: Option<GridRect>,
    zoom: f32,
    target_zoom: f32,
}

#[derive(Debug, Default, Clone)]
pub enum State {
    Dragging {
        start: egui::Pos2,
        top_left_at_start: egui::Pos2,
        target: egui::Pos2,
        //rect: Option<GridRect>,
        rect: GridRect,
        do_copy: bool,
    },
    Selecting {
        start: egui::Pos2,
        top_left_at_start: egui::Pos2,
        rect: Option<GridRect>,
    },
    #[default]
    Normal,
}

impl State {
    pub fn prevent_moving(&self) -> bool {
        match self {
            Self::Dragging { .. } => true,
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
        let mut used_cells = Vec::with_capacity((width * height).into());
        used_cells.resize_with((width * height).into(), Default::default);

        Self {
            id: egui::Id::NULL,
            cell_width: 128.0,
            cell_height: 32.0,
            cell_size: egui::Vec2::new(128.0, 32.0),
            width,
            height,
            cells,
            used_cells,
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
    pub fn set_target_grid_pos(&mut self, target_grid_pos: Option<&GridPos>) {
        self.target_rect = if let Some(target_grid_pos) = target_grid_pos {
            if let Some(mut target_rect) = self.target_rect.take() {
                target_rect.set_top_left(target_grid_pos);
                Some(target_rect)
            } else {
                Some(GridRect::from_top_left_with_size_one(target_grid_pos))
            }
        } else {
            None
        };
    }
    pub fn add_cell(&mut self, x: u16, y: u16, content: UiGridCell) {
        let offset = (y * self.width + x) as usize;
        if offset > self.cells.capacity() {
            return;
        }
        self.cells[offset] = Some(content);
        self.used_cells[offset] = Some(());
    }

    fn is_cell_empty(&self, x: u16, y: u16) -> bool {
        let offset = (y * self.width + x) as usize;
        if offset > self.used_cells.capacity() {
            eprintln!(
                "Out of range {x} {y} {offset} <? {}",
                self.used_cells.capacity()
            );
            true
        } else {
            //eprintln!("{offset}, {:?}", self.used_cells[offset]);
            self.used_cells[offset].is_none()
        }
    }

    /*
    pub fn select_cell(&mut self, pos: Option<&GridPos>) {
        self.selected_cell = pos.cloned();
    }
    */

    pub fn select_rect(&mut self, rect: Option<&GridRect>) {
        self.selected_rect = rect.cloned();
        if let Some(r) = &rect {
            if let Some(tr) = &mut self.target_rect {
                let size = r.size();
                tr.set_size(&size);
            }
        } else {
            self.target_rect = None;
        }
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
                    rect.bottom_right_exclusive().x() as f32, // + 1.0,
                    rect.bottom_right_exclusive().y() as f32, // + 1.0,
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
        let mut action = None;
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

            let dim = if let State::Selecting { ref rect, .. } = temp.state {
                if let Some(r) = rect {
                    let stroke = egui::Stroke::new(
                        9.0,
                        egui::Color32::from_rgba_unmultiplied(125, 125, 50, 255),
                    );
                    let fill = egui::Color32::from_rgba_unmultiplied(125, 125, 50, 16);
                    self.paint_highlight_rect(ui, Some(&stroke), Some(&fill), &r);
                }
                0.2
            } else {
                1.0
            };
            if let Some(rect) = &self.selected_rect {
                let stroke = egui::Stroke::new(
                    9.0,
                    egui::Color32::from_rgba_unmultiplied(175, 150, 50, (255.0 * dim) as u8),
                );
                let fill = egui::Color32::from_rgba_unmultiplied(175, 150, 50, (16.0 * dim) as u8);
                self.paint_highlight_rect(ui, Some(&stroke), Some(&fill), &rect);
                let stroke = egui::Stroke::new(
                    5.0,
                    egui::Color32::from_rgba_unmultiplied(95, 20, 20, (255.0 * dim) as u8),
                );
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
                            r.set_size(GridPos::one());
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
                            r.set_size(GridPos::one());
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
        let selected_rect = self.selected_rect.clone();
        if response.contains_pointer() {
            ui.ctx().input(|i| {
                //eprintln!("{:#?}", i.pointer );
                let mut s = temp.state().clone();
                match &mut s {
                    State::Normal => {
                        if i.pointer.button_pressed(egui::PointerButton::Primary) {
                            if let Some(p) = i.pointer.interact_pos() {
                                let gp = self.screen_pos_to_grid_pos(&ui.min_rect().min, &p);
                                let inside_current_selection = selected_rect
                                    .clone()
                                    .map(|r| r.contains_pos(&gp))
                                    .unwrap_or(false);
                                if inside_current_selection {
                                    eprintln!("Start dragging");
                                    temp.set_state(State::Dragging {
                                        start: p.clone(),
                                        top_left_at_start: ui.min_rect().min.clone(),
                                        rect: selected_rect.unwrap().clone(),
                                        target: p.clone(),
                                        do_copy: false,
                                    });
                                } else {
                                    eprintln!("Start selection");
                                    temp.set_state(State::Selecting {
                                        start: p.clone(),
                                        top_left_at_start: ui.min_rect().min.clone(),
                                        rect: None,
                                    });
                                }
                            }
                        }
                    }
                    State::Dragging {
                        start,
                        top_left_at_start,
                        rect: new_dragging_gr,
                        target,
                        do_copy,
                    } => {
                        if i.key_pressed(Key::Escape) {
                            eprintln!("Dragging - cancelled");
                            temp.set_state(State::Normal);
                        } else if i.pointer.button_released(egui::PointerButton::Primary) {
                            // :TODO: end dragging
                            eprintln!("Dragging - button released");
                            temp.set_state(State::Normal);
                            if *do_copy {
                                // :TODO modifier for cloning
                                action = Some(UiGridAction::Copy {
                                    source_rect: new_dragging_gr.clone(),
                                    target_pos: self
                                        .screen_pos_to_grid_pos(&ui.min_rect().min, &target),
                                });
                            } else {
                                action = Some(UiGridAction::Move {
                                    source_rect: new_dragging_gr.clone(),
                                    target_pos: self
                                        .screen_pos_to_grid_pos(&ui.min_rect().min, &target),
                                });
                            }
                        } else {
                            if let Some(p) = i.pointer.interact_pos() {
                                eprintln!("Dragging {p:?}");
                                temp.set_state(State::Dragging {
                                    start: *start,
                                    top_left_at_start: *top_left_at_start,
                                    rect: new_dragging_gr.clone(),
                                    target: p.clone(),
                                    do_copy: i.modifiers.alt,
                                });
                            } else {
                                eprintln!("Dragging - no position");
                            }
                        }
                    }
                    State::Selecting {
                        start,
                        top_left_at_start,
                        rect: new_selection_gr,
                    } => {
                        if let Some(current) = i.pointer.interact_pos() {
                            let delta = ui.min_rect().min - *top_left_at_start;
                            /*
                            eprintln!(
                                "Top Left at start {top_left_at_start:?} current {:?} -> {delta:?}",
                                ui.min_rect().min
                            );
                            */
                            let delta_start = *start + delta;
                            let r = egui::Rect::from_two_pos(delta_start, current);
                            // calculate potential new selection
                            let mut gtl = self.screen_pos_to_grid_pos(&ui.min_rect().min, &r.min);
                            let gbr = self.screen_pos_to_grid_pos(&ui.min_rect().min, &r.max);

                            gtl.inc_x();
                            gtl.inc_y();
                            //gbr.dec_x();
                            //gbr.dec_y();

                            //eprintln!("{gtl:?} {gbr:?}");
                            let mut gr = GridRect::new(gtl, gbr);
                            // shrink to minum needed
                            // cut of top
                            let mut first_non_empty_row = u16::MAX;

                            'scan_rows: for gy in gr.top_left().y()..gr.bottom_right_exclusive().y()
                            {
                                //eprintln!("gy: {gy}");
                                for gx in gr.top_left().x()..gr.bottom_right_exclusive().x() {
                                    //eprintln!("gx: {gx}");
                                    if !self.is_cell_empty(gx, gy) {
                                        first_non_empty_row = gy;
                                        //eprintln!("Non empty cell at {gx} {gy}");
                                        break 'scan_rows;
                                    }
                                }
                            }

                            if first_non_empty_row < u16::MAX {
                                gr.set_top(first_non_empty_row);
                            } else {
                                // rect is empty
                                gr.set_size(GridPos::zero());
                            }
                            // cut of bottom
                            let mut last_non_empty_row = u16::MIN;

                            'scan_rows: for gy in (gr.top_left().y()
                                ..gr.bottom_right_exclusive().y())
                                .into_iter()
                                .rev()
                            {
                                //eprintln!("gy: {gy} [bottom]");
                                for gx in gr.top_left().x()..gr.bottom_right_exclusive().x() {
                                    //eprintln!("gx: {gx}");
                                    if !self.is_cell_empty(gx, gy) {
                                        last_non_empty_row = gy;
                                        //eprintln!("Non empty cell at {gx} {gy} [bottom]");
                                        break 'scan_rows;
                                    }
                                }
                            }

                            if last_non_empty_row > u16::MIN {
                                gr.set_bottom_inclusive(last_non_empty_row);
                            } else {
                                // rect is empty
                                //eprintln!("Rect is empty! [bottom] {last_non_empty_row}");
                                gr.set_size(GridPos::zero());
                            }
                            // cut of left
                            let mut first_non_empty_col = u16::MAX;

                            'scan_cols: for gx in gr.top_left().x()..gr.bottom_right_exclusive().x()
                            {
                                for gy in gr.top_left().y()..gr.bottom_right_exclusive().y() {
                                    if !self.is_cell_empty(gx, gy) {
                                        first_non_empty_col = gx;
                                        //eprintln!("Non empty cell at {gx} {gy}");
                                        break 'scan_cols;
                                    }
                                }
                            }
                            // eprintln!( "{gr:?} Size {:?} | before cutting {first_non_empty_col}",gr.size());

                            if first_non_empty_col < u16::MAX {
                                gr.set_left(first_non_empty_col);
                            } else {
                                // rect is empty
                                gr.set_size(GridPos::zero());
                            }

                            // cut of right
                            let mut last_non_empty_col = u16::MIN;

                            'scan_cols: for gx in (gr.top_left().x()
                                ..gr.bottom_right_exclusive().x())
                                .into_iter()
                                .rev()
                            {
                                for gy in gr.top_left().y()..gr.bottom_right_exclusive().y() {
                                    if !self.is_cell_empty(gx, gy) {
                                        last_non_empty_col = gx;
                                        // eprintln!("Non empty cell at {gx} {gy} [right]");
                                        break 'scan_cols;
                                    }
                                }
                            }

                            if last_non_empty_col > u16::MIN {
                                gr.set_right_inclusive(last_non_empty_col);
                            } else {
                                // rect is empty
                                //eprintln!("Rect is empty! [right] {last_non_empty_col}");
                                gr.set_size(GridPos::zero());
                            }

                            //eprintln!("{gr:?} Size {:?}", gr.size());
                            let sr = if gr.size().x() == 0 || gr.size().y() == 0 {
                                None
                            } else {
                                Some(gr)
                            };
                            temp.set_state(State::Selecting {
                                start: *start,
                                top_left_at_start: *top_left_at_start,
                                rect: sr,
                            });
                            rect = Some(r);
                        }
                        if i.pointer.button_released(egui::PointerButton::Primary) {
                            //let new_selection_gr = new_selection_gr.clone();
                            // eprintln!("End selection - {new_selection_gr:?}");
                            temp.set_state(State::Normal);
                            if selected_grid_rect.is_none() {
                                selected_grid_rect = new_selection_gr.take();
                            }
                        }
                    }
                    _ => {
                        eprintln!("State: {:?}", temp.state());
                    }
                }
            });
        } else {
            // :TODO: handle pointer outside while dragging/selecting
        }

        match temp.state() {
            State::Dragging {
                start,
                top_left_at_start,
                rect: grid_rect,
                target,
                do_copy,
            } => {
                let rounding = 0.0;
                let stroke = egui::Stroke::new(2.25, egui::Color32::from_rgb(250, 150, 100));
                let dim = 1.0;
                let fill = egui::Color32::from_rgba_unmultiplied(175, 75, 25, (127.0 * dim) as u8);
                let rect_size = self.cell_size
                    * self.zoom
                    * egui::Vec2::new(grid_rect.size().x() as f32, grid_rect.size().y() as f32);
                let rect_pos = target; // + p.to_vec2();
                let rect_pos = rect_pos.to_vec2();
                let rect_pos = rect_pos - ui.min_rect().min.to_vec2();
                let rect_pos = rect_pos / (self.cell_size * self.zoom);
                let rect_pos = rect_pos.floor();
                let rect_pos = rect_pos * (self.cell_size * self.zoom);
                let rect_pos = rect_pos.to_pos2();
                let rect_pos = rect_pos + ui.min_rect().min.to_vec2();
                let rect = egui::Rect::from_min_size(rect_pos, rect_size);
                ui.painter().rect_filled(rect, rounding, fill);
                ui.painter().rect_stroke(rect, rounding, stroke);
                //let cell_pos = ui.min_rect().min + target.to_vec2(); // + p.to_vec2();
                let cell_pos = target; // + p.to_vec2();
                let cell_pos = *cell_pos - egui::Vec2::new(10.0, 10.0);

                let cell_rect = egui::Rect::from_center_size(cell_pos, self.cell_size);
                let cell_rect = cell_rect.shrink(1.0);
                if *do_copy {
                    ui.put(cell_rect, egui::Label::new("Clone"));
                } else {
                    ui.put(cell_rect, egui::Label::new("Move"));
                }
            }
            _ => {}
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
                r.set_size(GridPos::one());
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
            target_grid_rect: self.target_rect.clone(),
            prevent_moving: temp.state().prevent_moving(),
            action,
        }
    }
}

impl Widget for UiGrid {
    fn ui(self, ui: &mut Ui) -> Response {
        // todo!();
        self.show(ui).response
    }
}
