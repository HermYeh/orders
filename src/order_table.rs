use chrono::{DateTime, Local};
use egui::{Color32, Label, Stroke, TextStyle};

use crate::TemplateApp;

pub struct Table {
    striped: bool,
    num_rows: usize,
    clickable: bool,
 
   
}
impl Default for Table {
    fn default() -> Self {
        Self {
           
            striped: true,
            clickable: true,
            num_rows: 10_000,
        
            
        }
    }
}

impl Table {
    
   pub fn table_ui(&mut self, ui: &mut egui::Ui,table_data:&mut TemplateApp) {
        use egui_extras::{Column, TableBuilder};

   
   
    let available_height = ui.available_height();
    let mut table = TableBuilder::new(ui).cell_layout(egui::Layout::left_to_right(egui::Align::LEFT))
        .column(Column::auto())
        .column(Column::exact(80.00))
        .column(Column::exact(80.00))
        .column(Column::exact(80.00))
        .resizable(true)
        .striped(self.striped)
        .min_scrolled_height(0.0)
        .max_scroll_height(available_height);
        
        table = table.sense(egui::Sense::click());
        if let Some(row_index) = table_data.scroll_to_row.take() {
            table = table.scroll_to_row(row_index, None);
        }

        
    table.header(20.0, |mut header| {
        header.col(|ui| {
            ui.heading("Order#");
        });
        header.col(|ui| {
            ui.heading("Check In");
        });
        header.col(|ui| {
            ui.heading("Wait Time");
        });
        header.col(|ui| {
            ui.heading("Payment");
        })
        ;
        })
        
    .body(|mut body| {
    let mut order_size=table_data.total_order.len();
    for row_index in 0..order_size {    
        if row_index>=table_data.total_order.len(){
            continue;
         }
        body.row(20.0, |mut row| {
            let rowindex=row.index();
            if table_data.selection==row_index{
                row.set_selected(true);
            }else {
                row.set_selected(false);
            }
            
            row.col(|ui| {
                ui.add_sized(ui.available_size(),Label::new(egui::RichText::new(table_data.total_order[rowindex].0.clone()).size(20.0)).selectable(false),);
            });
            row.col(|ui| {
                ui.add_sized(ui.available_size(),Label::new(egui::RichText::new(table_data.total_order[rowindex].1.clone().format("%H:%M").to_string()).size(20.0)).selectable(false),);
            });
            row.col(|ui| {
                let time_now: DateTime<Local> = Local::now();
                let time_wait = time_now-table_data.total_order[rowindex].1.clone();
                let minutes = (time_wait.num_minutes()).to_string();
               
               
               let time= minutes+" min";
                ui.add_sized( ui.available_size(),Label::new(egui::RichText::new(time).size(20.0)).selectable(false),);
            });
            row.col(|ui| {
                let response = ui
                .add_sized(
                    ui.available_size(),
                    egui::Button::new(if table_data.payment[rowindex] {"Paid"}else{""}),
                );
                if response.clicked(){
                 
                    table_data.payment[rowindex] =!table_data.payment[rowindex];
                  
                }
            });
            
            toggle_row_selection(table_data,row_index, &row.response());
         
        });
       
        };
        body.row(20.0, |mut row| {
            if table_data.selection==table_data.total_order.len(){
                row.set_selected(true);
            }else {
                row.set_selected(false);
            }
            row.col(|ui| {
                ui.add_sized(ui.available_size(),Label::new(egui::RichText::new(table_data.order_number.concat()).size(20.0)).selectable(false),);
            });
           

        });
        
   
       
    });
    }
  
}  
fn toggle_row_selection(select:&mut TemplateApp, row_index: usize, row_response: &egui::Response) {
    if row_response.clicked() {
        select.selection=row_index;
    }
 
}
