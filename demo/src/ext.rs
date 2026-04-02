use eframe::egui::{Frame, Ui};
use extension_traits::extension;

#[extension(pub trait UiExt)]
impl Ui {
    fn choice_frame<T, I>(&mut self, label: &str, state: &mut T, options: I)
    where
        T: std::fmt::Debug + PartialEq,
        I: IntoIterator<Item = T>,
    {
        Frame::window(self.style()).show(self, |ui| {
            ui.label(label);
            ui.separator();
            for option in options {
                let text = format!("{:?}", &option);
                if ui.selectable_value(state, option, text).clicked() {
                    ui.close();
                    return;
                }
            }
        });
    }
}
