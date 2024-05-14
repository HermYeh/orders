
use egui::{ColorImage, Image};
use std::fs;
use egui_extras::RetainedImage;
use std::time::Duration;
use egui_extras::{TableBuilder, Column};
use chrono::{DateTime, Local};
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
    total_order:Vec<String>,
    order_time:Vec<String>,
    rows: i32,
    row_index: i32,
    friedbun_count: i32,

}

impl<'a> Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            order_number:Vec::new(),
            total_order:Vec::new(),
            order_time:Vec::new(),
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
       /*  if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        } */
   
    
        Default::default()
    }
    
}
fn check_order(template_app:&mut TemplateApp){
     
    if template_app.order_number.len()==4{
        template_app.total_order.push(template_app.order_number.concat());
        let time: DateTime<Local> = Local::now();
        template_app.order_time.push(time.format("%H:%M").to_string());
        template_app.order_number.clear();
    };
}

impl<'a>  eframe::App for TemplateApp  {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
      /*   eframe::set_value(storage, eframe::APP_KEY, self); */
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
     
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
    
        
                let table = TableBuilder::new(ui).cell_layout(egui::Layout::top_down(egui::Align::LEFT))
                .column(Column::auto())
                .column(Column::exact(80.00))
                .striped(true)
                .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Order#");
                });
                header.col(|ui| {
                    ui.heading("Time");
                });
                })
            .body(|mut body| {
               
                for row_index in 0..self.total_order.len() {    
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                   
                        ui.label(
                            egui::RichText::new(self.total_order[row_index].clone()).size(20.0)
                           
                        );
                        
                    });
                    row.col(|ui| {
                        ui.label(
                            egui::RichText::new(self.order_time[row_index].clone()).size(20.0)
                        );
                    });
                });
                };
                body.row(20.0, |mut row| {
                    row.col(|ui| {
                   
                        ui.label(
                            egui::RichText::new(self.order_number.concat()).size(20.0)
                           
                        );
                        
                    });
              
                });

            });
            
          

            
        });
     
       
  
     /*    if self.order_number.len()==4{
            self.total_order.push(self.order_number.concat());
            let time: DateTime<Local> = Local::now();
            self.order_time.push(time.format("%H:%M").to_string());
            self.order_number.clear();
        }; */
        
        egui::SidePanel::right("right").show(ctx, |ui| {

            ui.with_layout(egui::Layout::top_down(egui::Align::BOTTOM), |ui| {
                
       
        ui.vertical(|ui|    {         
            ui.horizontal(|ui| {     
            
                for but_index in 1..4{
                    let button = ui.add_sized(
                        [50.0,50.0],
                        egui::Button::new(but_index.to_string())
                    ) ;
                    if button.clicked(){
                        self.order_number.push(but_index.to_string());
                        check_order(self);
                    }
                }
               
            });
            ui.horizontal(|ui| {     
                for but_index in 4..7{
                    let button = ui.add_sized(
                        [50.0,50.0],
                        egui::Button::new(but_index.to_string())
                    ) ;
                    if button.clicked(){
                        self.order_number.push(but_index.to_string());
                        check_order(self);
                    }
                }
               
            });  
            ui.horizontal(|ui| {     
                for but_index in 7..10{
                    let button = ui.add_sized(
                        [50.0,50.0],
                        egui::Button::new(but_index.to_string())
                    ) ;
                    if button.clicked(){
                        self.order_number.push(but_index.to_string());
                        check_order(self);
                    }
                }
               
            });    
            ui.horizontal(|ui| {     
          
                    let button = ui.add_sized(
                        [150.0,50.0],
                        egui::Button::new("0".to_string())
                    ) ;
                    if button.clicked(){
                        self.order_number.push("0".to_string());
                        check_order(self);
                    }
              
               
            });  
        }); 
    });   
    });
     ctx.request_repaint();
     std::thread::sleep(Duration::from_millis(1));
    }
}

