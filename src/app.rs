
use egui::{ColorImage, Image};
use std::fs;
use egui_extras::RetainedImage;
use std::time::Duration;
use egui_extras::{TableBuilder, Column};

use std::path::Path;
use eframe::{egui};
use egui::{ Id, RichText, TextureHandle, Vec2};
use image::{self, RgbImage, ImageBuffer, open};
use std::sync::mpsc::channel;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    
    #[serde(skip)] // This how you opt-out of serialization of a field
    order_number: Vec<String>,
    rows: i32,
    row_index: i32,
    friedbun_count: i32,

}

impl<'a> Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            order_number:vec!["".to_owned()],
            rows: 1,
            row_index: 0,
            friedbun_count: 0,
          
        }
    }
}

impl  TemplateApp  {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
   
    
        Default::default()
    }
}
pub fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

impl<'a>  eframe::App for TemplateApp  {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui_extras::install_image_loaders(ctx);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            
      
        
      /*   let timer = timer::Timer::new();
        let (tx, rx) = channel();
        
        let _guard = timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
        tx.send(()).unwrap();
        });

        rx.recv().unwrap();
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }
                
                egui::widgets::global_dark_light_mode_buttons(ui);
            }); */
        });
        egui::CentralPanel::default().show(ctx, |ui| {
    
        
                let table = TableBuilder::new(ui).cell_layout(egui::Layout::left_to_right(egui::Align::TOP))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .striped(true)
                .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Order#");
                });
                header.col(|ui| {
                    ui.heading("Time Checkin");
                });
                })
            .body(|mut body| {
          
                for row_index in 0..self.rows {    
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        
                        ui.label(
                            egui::RichText::new(self.order_number[0].clone()).size(20.0)
                           
                        );
                        
                    });
              
                });
                };
            });
            
           
        

            
        });
     
       
 
        
        egui::TopBottomPanel::bottom("bot").show(ctx, |ui| {
            
           
        ui.vertical(|ui| {         
            ui.horizontal(|ui| {     
                let button_1 = ui.add_sized(
                    [50.0,50.0],
                    egui::Button::new("1")
                ) ;
                if button_1.clicked(){
                    self.order_number.push("1".to_string());
                }
                
                let button_2 = ui.add_sized(
                    [50.0,50.0],
                    egui::Button::new("2")
                ) ;
                if button_2.clicked(){
                    self.order_number.push("1".to_string());
                }
                let button_3 = ui.add_sized(
                    [50.0,50.0],
                    egui::Button::new("3")
                ) ;
                if button_3.clicked(){
                    self.order_number.push("1".to_string());
                }
            });
            ui.horizontal(|ui| {     
                let button_4 = ui.add_sized(
                    [50.0,50.0],
                    egui::Button::new("4")
                ) ;
                if button_4.clicked(){
                    self.order_number.push("1".to_string());
                }
                
                let button_5 = ui.add_sized(
                    [50.0,50.0],
                    egui::Button::new("5")
                ) ;
                if button_5.clicked(){
                    self.order_number.push("1".to_string());
                }
                let button_6 = ui.add_sized(
                    [50.0,50.0],
                    egui::Button::new("6")
                ) ;
                if button_6.clicked(){
                    self.order_number.push("1".to_string());
                }
            });  
            ui.horizontal(|ui| {     
                let button_7 = ui.add_sized(
                    [50.0,50.0],
                    egui::Button::new("7")
                ) ;
                if button_7.clicked(){
                    self.order_number.push("1".to_string());
                }
                
                let button_8 = ui.add_sized(
                    [50.0,50.0],
                    egui::Button::new("8")
                ) ;
                if button_8.clicked(){
                    self.order_number.push("1".to_string());
                }
                let button_9 = ui.add_sized(
                    [50.0,50.0],
                    egui::Button::new("9")
                ) ;
                if button_9.clicked(){
                    self.order_number.push("1".to_string());
                }
            });    
        }); 
        
            
     });
     ctx.request_repaint();
     std::thread::sleep(Duration::from_millis(1));
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
