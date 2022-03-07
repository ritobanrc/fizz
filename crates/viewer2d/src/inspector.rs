use egui::Ui;

pub trait EguiInspector {
    fn egui_update(&mut self, ui: &mut Ui);
}

impl<T: egui::emath::Numeric + na::Scalar, D: na::Dim, S: na::storage::StorageMut<T, D>>
    EguiInspector for na::Vector<T, D, S>
{
    fn egui_update(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for x in self.iter_mut() {
                // TODO: support different speed values
                ui.add(egui::DragValue::new(x).speed(0.01));
            }
        });
    }
}

impl<T: EguiInspector> EguiInspector for fizz::base::Range<T> {
    fn egui_update(&mut self, ui: &mut Ui) {
        ui.label("\tMin");
        self.min.egui_update(ui);

        ui.end_row();

        ui.label("\tMax");
        self.max.egui_update(ui);
        ui.end_row();
    }
}
