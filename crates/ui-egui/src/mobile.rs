//! Touch-first controls for narrow browser windows (especially Android phones).
use crate::{SlideApp, theme};
use serde_json::json;

fn action(app: &mut SlideApp, id: &str) {
    let result = if id == "edit.delete" {
        app.session
            .key("Delete", deckcraft_engine::Mods { shift: false, alt: false, cmd: false })
            .map(|_| serde_json::Value::Null)
            .map_err(|err| err.to_string())
    } else {
        app.run(id, json!({}))
    };
    if let Err(err) = result {
        app.notify(err);
    }
}

pub fn is_mobile(ctx: &egui::Context) -> bool {
    ctx.input(|input| input.viewport_rect().width() < 960.0 || input.viewport_rect().height() < 520.0)
}

pub fn header(app: &mut SlideApp, ui: &mut egui::Ui) {
    let dark = app.ui.brightness == theme::Brightness::Dark;
    let t = theme::Tokens::for_brightness(app.ui.brightness);
    ui.horizontal(|ui| {
        ui.add_space(4.0);
        ui.label(egui::RichText::new("DC").strong().size(20.0).color(t.accent));
        ui.label(egui::RichText::new("DeckCraft").strong().size(16.0).color(t.text));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let theme_label = if dark { "☀" } else { "☾" };
            if ui
                .add_sized([44.0, 44.0], egui::Button::new(egui::RichText::new(theme_label).size(21.0)).corner_radius(14.0))
                .on_hover_text(if dark { "Ativar modo claro" } else { "Ativar modo escuro" })
                .clicked()
                && let Err(err) = app.run("view.dark", json!({"on": !dark}))
            {
                app.notify(err.to_string());
            }
            if ui
                .add_sized([44.0, 44.0], egui::Button::new(egui::RichText::new("⌕").size(23.0)).corner_radius(14.0))
                .on_hover_text("Pesquisar comandos")
                .clicked()
            {
                app.palette = Some((String::new(), 0));
            }
            if ui.add_sized([44.0, 44.0], egui::Button::new("⋯").corner_radius(14.0)).on_hover_text("Mais opções").clicked() {
                app.palette = Some((String::new(), 0));
            }
        });
    });
    ui.add_space(2.0);
    ui.separator();
}

fn tool_button(ui: &mut egui::Ui, label: &str, hint: &str) -> bool {
    ui.add_sized([66.0, 54.0], egui::Button::new(egui::RichText::new(label).size(13.0)).corner_radius(13.0)).on_hover_text(hint).clicked()
}

pub fn actions(app: &mut SlideApp, ui: &mut egui::Ui) {
    let t = theme::Tokens::get(ui.ctx());
    egui::Frame::NONE.fill(t.chrome).inner_margin(egui::Margin { left: 6, right: 6, top: 5, bottom: 5 }).show(ui, |ui| {
        egui::ScrollArea::horizontal().auto_shrink([false, true]).show(ui, |ui| {
            ui.horizontal(|ui| {
                if tool_button(ui, "＋\nSlide", "Adicionar slide") {
                    action(app, "slide.new");
                }
                if tool_button(ui, "↶\nDesfazer", "Desfazer") {
                    action(app, "edit.undo");
                }
                if tool_button(ui, "↷\nRefazer", "Refazer") {
                    action(app, "edit.redo");
                }
                if tool_button(ui, "▢\nCopiar", "Copiar objeto ou seleção") {
                    action(app, "edit.copy");
                }
                if tool_button(ui, "▣\nDuplicar", "Duplicar seleção") {
                    action(app, "edit.duplicate");
                }
                if tool_button(ui, "▤\nColar", "Colar") {
                    action(app, "edit.paste");
                }
                if tool_button(ui, "⌫\nApagar", "Apagar objeto selecionado") {
                    action(app, "edit.delete");
                }
                if tool_button(ui, "▶\nApresentar", "Iniciar apresentação") {
                    let from = app.session.active().map(|d| d.selection.slide).unwrap_or(0);
                    app.start_show(from, false);
                }
            });
        });
    });
    egui::ScrollArea::horizontal().auto_shrink([false, true]).show(ui, |ui| {
        ui.horizontal(|ui| {
            for (label, tab) in [
                ("Início", "Home"),
                ("Inserir", "Insert"),
                ("Design", "Design"),
                ("Transições", "Transitions"),
                ("Animações", "Animations"),
                ("Organizar", "Arrange"),
            ] {
                let selected = app.ui.tab == tab;
                if ui.add_sized([82.0, 42.0], egui::Button::new(egui::RichText::new(label).color(t.text)).selected(selected)).clicked() {
                    app.ui.tab = tab.to_string();
                }
            }
        });
    });
    ui.separator();
}
