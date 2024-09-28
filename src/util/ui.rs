use egui::FontId;

pub const DEFAULT_FONT: FontId = egui::FontId {
    family: egui::FontFamily::Monospace,
    size: 14.0,
};

#[macro_export]
macro_rules! rich_text {
    ($text:expr) => {{
        let uniq = crate::util::Uniq::Module({
            let module_path = module_path!();
            module_path.splitn(3, ':').nth(2).unwrap_or(module_path)
        });
        let text: String = $text.into();
        egui::WidgetText::RichText(
            egui::RichText::new(crate::util::trans(text.into(), uniq))
                .font(crate::util::DEFAULT_FONT.clone()),
        )
    }};
}
