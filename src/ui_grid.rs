use crate::UiGridCell;
use egui::Response;
use egui::Ui;
use egui::Widget;
#[derive(Debug)]
pub struct UiGridOutput {
    selected: Option<(u16, u16)>,
    response: Response,
}

impl UiGridOutput {
    pub fn selected(&self) -> Option<(u16, u16)> {
        self.selected
    }
}

//#[derive(Debug)]
pub struct UiGrid {
    cell_width: f32,
    cell_height: f32,
    width: u16,
    height: u16,
    //cells: Vec<Option<String>>, // :HACK: this should be sparse
    cells: Vec<Option<UiGridCell>>, // :HACK: this should be sparse
}

impl Default for UiGrid {
    fn default() -> Self {
        let width = 32u16;
        let height = 32u16;
        let mut cells = Vec::with_capacity((width * height).into());
        cells.resize_with((width * height).into(), Default::default);

        Self {
            cell_width: 128.0,
            cell_height: 32.0,
            width,
            height,
            cells,
        }
    }
}

impl UiGrid {
    pub fn add_cell(&mut self, x: u16, y: u16, content: UiGridCell) {
        let offset = (y * self.width + x) as usize;
        if offset > self.cells.capacity() {
            return;
        }
        self.cells[offset] = Some(content);
    }

    pub fn show(self, ui: &mut Ui) -> UiGridOutput {
        let desired_size = egui::vec2(
            self.cell_width * self.width as f32,
            self.cell_height * self.height as f32,
        );
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        let mut selected = None;
        if ui.is_rect_visible(rect) {
            let cell_size = egui::Vec2::new(self.cell_width, self.cell_height);

            // paint grid
            let stroke = egui::Stroke::new( 0.25, egui::Color32::from_rgb( 50, 50, 50 ));
	        // let visuals = ui.style().interact_selectable(&response, true);

            let vis_r = response.interact_rect;//.translate( ui.min_rect().min.to_vec2() );
            let c = ( ( vis_r.width() / self.cell_width ).ceil() ) as usize;
            let c = c + 1;
            let ul = ui.min_rect().min; // upper left of "window"
            let lr = ui.min_rect().max; // lower right of "window"

            let p = vis_r.min - ul;
            let p = p / egui::Vec2::new( self.cell_width, self.cell_height );
            let p = p.floor();
            let p = p * egui::Vec2::new( self.cell_width, self.cell_height );
            let mut p = ul + p;

            // vertical lines
            for _ in 0..c {
				ui.painter().vline(
					p.x,
					egui::Rangef::new( ul.y, lr.y ),
		            stroke, //visuals.bg_stroke,
        		);
        		p.x += self.cell_width;
            }

            let c = ( ( vis_r.height() / self.cell_height ).ceil() ) as usize;
            let c = c + 1;
            for _ in 0..c {
				ui.painter().hline(
					egui::Rangef::new( ul.x, lr.x ),
					p.y,
		            stroke, //visuals.bg_stroke,
        		);
        		p.y += self.cell_height;
            }

            for (idx, content) in self.cells.into_iter().enumerate() {
                let y = idx / self.width as usize;
                let x = idx % self.width as usize;
                let cell_pos = egui::Pos2::new(
                    self.cell_width * ((x as f32) + 0.5),
                    self.cell_height * ((y as f32) + 0.5),
                );
                let cell_pos = ui.min_rect().min + cell_pos.to_vec2();
                let cell_rect = egui::Rect::from_center_size(cell_pos, cell_size);

                if let Some(content) = content {
                    //let r = ui.put(cell_rect, UiGridCell::new(content.to_string()));
                    //let content = Box::new( egui::Label::new("fii") );
                    let r = ui.put(cell_rect, content);

                    if r.clicked() {
                        // eprintln!("Clicked {x}, {y} {content}");
                        selected = Some((x as u16, y as u16));
                    }
                } else {
                }
            }
        }

        UiGridOutput { selected, response }
    }
}

impl Widget for UiGrid {
    fn ui(self, ui: &mut Ui) -> Response {
        // todo!();
        self.show(ui).response
    }
}
