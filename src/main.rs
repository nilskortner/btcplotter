use plotters::prelude::*;
use serde::Serialize;
use csv::Reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/5.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE);
    let root = root.margin(10, 10, 20, 20);
    // After this point, we should be able to construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("This is our first plot", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..900_000f32, 70_000f32..120_000f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    let data = read_csvs(0, 21);
    let prices = data.iter().map(|record| record.price as f32);
    let mut plotdata: Vec<(f32,f32)> = Vec::new();
    let mut i :f32 = 0.0;

    for price in prices{
        let tuple = (i,price);
        plotdata.push(tuple);
        i += 10.0;
    }

    // // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        plotdata,
        &RED,
    ))?;
    //Similarly, we can draw point series
    // chart.draw_series(PointSeries::of_element(
    //     plotdata,
    //     1,
    //     &RED,
    //     &|c, s, st| {
    //         return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
    //         + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
    //         + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
    //     },
    // ))?;
    root.present()?;
    Ok(())
}

pub fn read_csvs(start: i32, end: i32) -> Vec<Record> {
    let mut start = start;
    let mut records: Vec<Record> = Vec::new();
    while start <= end  {
        let name = format!("outputs/output{}.csv", start);
        let mut reader = Reader::from_path(name).unwrap();
        read_csv(&mut records, &mut reader);
        start+=1;
    }

    records
}

#[derive(Serialize)]
pub struct Record {
    pub price: f64,
    pub depthbids: String,
    pub depthasks: String,
    time: String,
}

impl Record {
    // Helper method to convert the struct fields to a Vec of strings
    fn to_record(&self) -> Vec<String> {
        vec![self.price.to_string(), self.depthbids.clone(), self.depthasks.clone() ,self.time.clone()]
    }
}

pub fn read_csv<R: std::io::Read>(records: &mut Vec<Record>, reader: &mut Reader<R>) {

    for result in reader.records() {
        match result {
            Ok(read) => {
                let price = &read[0];
                let depthbids = read[1].to_string();
                let depthasks = read[2].to_string();
                let time = read[3].to_string();

                //println!("{} {} {} {}", price, depthbids, depthasks, time);
        
                let price = price.parse::<f64>().unwrap();
        
                let record = Record{ 
                    price: price,
                    depthasks : depthasks,
                    depthbids: depthbids,
                    time: time,
                };
                records.push(record);

            }
            Err(e) => { println!("{}", e);}
        }
    }
}