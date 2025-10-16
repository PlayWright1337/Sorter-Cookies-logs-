mod sorter;
use sorter::{extract_cookies, SaveMode};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 650.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Cookie Extractor",
        options,
        Box::new(|cc| {
            // Настраиваем темную тему с более приятными цветами
            let mut visuals = egui::Visuals::dark();
            visuals.window_rounding = 10.0.into();
            visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 30, 40);
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 45, 60);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(55, 55, 75);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(65, 65, 85);
            visuals.selection.bg_fill = egui::Color32::from_rgb(60, 100, 180);
            cc.egui_ctx.set_visuals(visuals);
            Box::<ExtractorCookies>::default()
        }),
    )
}

#[derive(Default, PartialEq)]
enum Tab {
    #[default]
    Sorter,
    Settings,
    AboutMe,
}

#[derive(Default)]
struct ExtractorCookies {
    active_tab: Tab,
    button_clicked: bool,
    query: String,
    save_mode: SaveMode,
    logs_path: Option<String>,
}

impl eframe::App for ExtractorCookies {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tabs_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(20.0, 0.0);
                ui.add_space(10.0);

                let tabs = [
                    ("Sorter", Tab::Sorter),
                    ("Settings", Tab::Settings),
                    ("About me", Tab::AboutMe),
                ];

                for (name, tab) in tabs {
                    let selected = self.active_tab == tab;
                    let (color_bg, color_border, color_text) = if selected {
                        (
                            egui::Color32::from_rgb(60, 120, 255),
                            egui::Color32::from_rgb(100, 150, 255),
                            egui::Color32::WHITE,
                        )
                    } else {
                        (
                            egui::Color32::from_rgb(40, 40, 40),
                            egui::Color32::from_rgb(80, 80, 80),
                            egui::Color32::from_rgb(180, 180, 180),
                        )
                    };

                    let response = ui.add(
                        egui::Button::new(
                            egui::RichText::new(name)
                                .color(color_text)
                                .font(egui::FontId::proportional(16.0)),
                        )
                        .fill(color_bg)
                        .stroke(egui::Stroke::new(1.0, color_border))
                        .min_size(egui::vec2(100.0, 36.0))
                        .rounding(8.0),
                    );

                    if response.clicked() {
                        self.active_tab = tab;
                    }
                }

                ui.add_space(10.0);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.active_tab {
            Tab::Sorter => {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading(egui::RichText::new("🍪 Cookie Sorter").size(28.0).strong());
                    ui.add_space(20.0);

                    // Создаем красивую рамку для основного контента
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(35, 35, 50))
                        .rounding(10.0)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 100)))
                        .inner_margin(16.0)
                        .show(ui, |ui| {
                            // Поле ввода сервисов с красивым оформлением
                            ui.add_space(5.0);
                            ui.label(egui::RichText::new("✨ Введите сервисы (через точку с запятой или Shift+Enter):").size(18.0).strong());
                            
                            let text_edit = egui::TextEdit::multiline(&mut self.query)
                                .hint_text("Например:\nsteam\nyoutube\nfacebook\n\nили: steam;youtube;facebook")
                                .desired_width(ui.available_width() * 0.95)
                                .desired_rows(4)
                                .margin(egui::vec2(10.0, 10.0))
                                .frame(true);
                            
                            let response = ui.add(text_edit);
                            
                            // Обработка Shift+Enter для добавления новой строки
                            if ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.shift) {
                                self.query.push('\n');
                            }
                            ui.add_space(15.0);

                            // Кнопка выбора папки с логами
                            let folder_button = egui::Button::new(
                                egui::RichText::new("📂 Выбрать папку с логами")
                                    .size(18.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(240, 240, 255))
                            )
                            .min_size(egui::vec2(ui.available_width() * 0.7, 42.0))
                            .fill(egui::Color32::from_rgb(70, 70, 120))
                            .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(120, 120, 180)))
                            .rounding(10.0);
                            
                            if ui.add_sized([ui.available_width(), 0.0], folder_button).clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                    self.logs_path = Some(path.display().to_string());
                                }
                            }

                            if let Some(path) = &self.logs_path {
                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("📁").size(16.0));
                                    ui.label(
                                        egui::RichText::new(format!("{}", path))
                                            .color(egui::Color32::from_rgb(180, 230, 180))
                                    );
                                });
                            }

                            ui.add_space(15.0);

                            // Группа режимов сохранения
                            egui::Frame::group(ui.style())
                                .fill(egui::Color32::from_rgb(40, 40, 60))
                                .rounding(8.0)
                                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 80, 120)))
                                .inner_margin(12.0)
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new("💾 Режим сохранения:").size(16.0));
                                    ui.add_space(8.0);
                                    
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 12.0;
                                        
                                        let button_size = egui::vec2(ui.available_width() / 3.0 - 8.0, 32.0);
                                        
                                        let full_log = self.save_mode == SaveMode::FullLog;
                                        if ui.add_sized(
                                            button_size,
                                            egui::SelectableLabel::new(
                                                full_log,
                                                egui::RichText::new("📋 Full Log")
                                                    .color(if full_log {
                                                        egui::Color32::WHITE
                                                    } else {
                                                        egui::Color32::LIGHT_GRAY
                                                    })
                                            )
                                        ).clicked() {
                                            self.save_mode = SaveMode::FullLog;
                                        }
                                        
                                        let folder_cookies = self.save_mode == SaveMode::FolderPlusCookies;
                                        if ui.add_sized(
                                            button_size,
                                            egui::SelectableLabel::new(
                                                folder_cookies,
                                                egui::RichText::new("📁 Folder+Cookies")
                                                    .color(if folder_cookies {
                                                        egui::Color32::WHITE
                                                    } else {
                                                        egui::Color32::LIGHT_GRAY
                                                    })
                                            )
                                        ).clicked() {
                                            self.save_mode = SaveMode::FolderPlusCookies;
                                        }
                                        
                                        let only_cookies = self.save_mode == SaveMode::OnlyCookies;
                                        if ui.add_sized(
                                            button_size,
                                            egui::SelectableLabel::new(
                                                only_cookies,
                                                egui::RichText::new("🍪 Only Cookies")
                                                    .color(if only_cookies {
                                                        egui::Color32::WHITE
                                                    } else {
                                                        egui::Color32::LIGHT_GRAY
                                                    })
                                            )
                                        ).clicked() {
                                            self.save_mode = SaveMode::OnlyCookies;
                                        }
                                    });
                                });
                        });

                    ui.add_space(20.0);

                    // Кнопка извлечения куки
                    let extract_button = egui::Button::new(
                        egui::RichText::new("🎯 Извлечь куки")
                            .size(20.0)
                            .color(egui::Color32::WHITE)
                            .strong()
                    )
                    .min_size(egui::vec2(200.0, 50.0))
                    .fill(egui::Color32::from_rgb(70, 130, 180))
                    .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(100, 160, 210)))
                    .rounding(10.0);
                    
                    let extract_response = ui.add_sized([ui.available_width() * 0.5, 0.0], extract_button);
                    
                    if extract_response.clicked() && !self.query.is_empty() {
                        if let Some(logs_path) = &self.logs_path {
                            // Обработка ввода: разделение по точке с запятой или по переносу строки
                            let queries: Vec<String> = self.query
                                .replace('\n', ";")
                                .split(';')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                            if let Err(err) = extract_cookies(logs_path, queries, self.save_mode.clone()) {
                                println!("Error: {:?}", err);
                            } else {
                                self.button_clicked = true;
                            }
                        }
                    }

                    ui.add_space(15.0);

                    if self.button_clicked {
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgba_premultiplied(0, 100, 0, 100))
                            .rounding(8.0)
                            .inner_margin(10.0)
                            .show(ui, |ui| {
                                ui.colored_label(
                                    egui::Color32::from_rgb(120, 255, 140),
                                    egui::RichText::new("✅ Куки успешно извлечены!")
                                        .size(16.0)
                                        .strong(),
                                );
                            });
                    }
                });
            }
            Tab::Settings => {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.heading(egui::RichText::new("⚙ Настройки").size(28.0).strong());
                    ui.add_space(20.0);
                    
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(35, 35, 50))
                        .rounding(10.0)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 100)))
                        .inner_margin(16.0)
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("Здесь вы можете настроить параметры приложения...").size(16.0));
                            ui.add_space(10.0);
                            ui.label("Дополнительные настройки будут доступны в следующих версиях.");
                        });
                });
            }
            Tab::AboutMe => {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.heading(egui::RichText::new("👤 О программе").size(28.0).strong());
                    ui.add_space(20.0);
                    
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(35, 35, 50))
                        .rounding(10.0)
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 100)))
                        .inner_margin(16.0)
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(10.0);
                                ui.label(egui::RichText::new("Cookie Extractor v1.0").size(20.0).strong());
                                ui.add_space(5.0);
                                ui.label(egui::RichText::new("Crated by Playwright1337 💙").size(16.0));
                                ui.add_space(15.0);
                                
                                ui.label("Программа для извлечения и сортировки cookie из логов");
                                ui.add_space(10.0);
                            });
                        });
                });
            }
        });

        ctx.request_repaint();
    }
}
