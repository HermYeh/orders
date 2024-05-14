
use egui::{ColorImage, Image, TextStyle, Ui};
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

use crate::order_table;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    
// This how you opt-out of serialization of a field
    pub order_number: Vec<String>,
    #[serde(skip)]
    pub total_order:Vec<(String, DateTime<Local>)>,
    pub order_time:Vec<String>,
    pub selection: usize,
    rows: i32,
    row_index: i32,
    friedbun_count: i32,
    pub payment: Vec<bool>,
    pub scroll_to_row: Option<usize>,
}

impl<'a> Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            order_number:Vec::new(),
            total_order:Default::default(),
            order_time:Vec::new(),
            selection: 999,
            rows: 1,
            row_index: 0,
            friedbun_count: 0,
            payment: vec![false;50],
        
            scroll_to_row: None,
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
        let time: DateTime<Local> = Local::now();
        template_app.total_order.push((template_app.order_number.concat(),time));
        template_app.order_number.clear();
    };
}
fn buttons(template_app:&mut TemplateApp,ui:&mut Ui){
     
    ui.horizontal(|ui| {     
            
        for but_index in 1..4{
            let button = ui.add_sized(
                [50.0,50.0],
                egui::Button::new(but_index.to_string())
            ) ;
            if button.clicked(){
                template_app.order_number.push(but_index.to_string());
                check_order(template_app);
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
                template_app.order_number.push(but_index.to_string());
                check_order(template_app);
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
                template_app.order_number.push(but_index.to_string());
                check_order(template_app);
            }
        }
       
    });    
    ui.horizontal(|ui| {     
  
            let button = ui.add_sized(
                [165.0,50.0],
                egui::Button::new("0".to_string())
            ) ;
            if button.clicked(){
                template_app.order_number.push("0".to_string());
                check_order(template_app);
            }
      
       
    });  
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
    
            let body_text_size = TextStyle::Body.resolve(ui.style()).size;
            use egui_extras::{Size, StripBuilder};
            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table
                .size(Size::exact(body_text_size)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            let mut table=order_table::Table::default();
                            table.table_ui(ui,self);
                        });
                    });
                  
                });
         
          
          
            
            
        });
     
  

        egui::SidePanel::right("right").show(ctx, |ui| {
            ui.add_space(360.0);
           /*  ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| { */
              buttons(self, ui)
                
         /*    });  */
  
    });
     ctx.request_repaint();
     std::thread::sleep(Duration::from_millis(1));
    }
}

