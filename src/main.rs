use fdm2d_sofc::*;
use ndarray::*;
use gnuplot::*;
use std::fs;
use std::io::Write;
use std::time::{Duration, SystemTime};
fn main() {
    let now = SystemTime::now();
    let l_ch = 20.;//Length of channel and cell 
    let h_el = 2.;// Thickness of electrolyte
    let h_fl = 1.;// Thickness of fuel electrode layer
    let h_al = 1.;// Thickness of air electrode layer
    let h_fch = 3.;// Thickness of fuel channel
    let h_ach = 3.;// Thickness of air channel

    let dx = 2.;
    let dy = 0.2;
//     let nx = (l_ch/dx) as usize;
     let nx = 20; 
    let ny = 10;
    let dt = 0.01;
    let nt = 1000;

    let mut ach = rectangle::Rectangle::new(0., 0.,
         l_ch, h_ach, nx, ny, dt,rectangle::RectangleType::Channel,vec![1.,0.1,1.,0.1]);
    let mut al = rectangle::Rectangle::new(0., h_ach,
         l_ch, h_al, nx, ny,dt, rectangle::RectangleType::Electrode,vec![0.1,0.1]);
    let mut el = rectangle::Rectangle::new(0., h_ach+h_al,
         l_ch, h_el, nx, ny, dt,rectangle::RectangleType::Electrolye,vec![0.1]);
    let mut fl = rectangle::Rectangle::new(0., h_ach+h_al+h_el,
         l_ch, h_fl, nx, ny, dt,rectangle::RectangleType::Electrode,vec![0.1,0.1]);
    let mut fch = rectangle::Rectangle::new(0., h_ach+h_al+h_el+h_fl,
         l_ch, h_fch, nx, ny, dt,rectangle::RectangleType::Channel,vec![1.,0.1,1.,0.1]);
    
    let data_zero = Array2::<f64>::zeros((ny,nx));
    let boundary_topbottom_zero=Array1::<f64>::zeros(nx);
    let boundary_leftright_zero=Array1::<f64>::zeros(ny);
    let mut fileu = fs::File::create("./data_u.csv").unwrap();
    let mut filev = fs::File::create("./data_v.csv").unwrap();
    let mut filep = fs::File::create("./data_p.csv").unwrap();
    let mut filec1= fs::File::create("./data_c1.csv").unwrap();
    let mut filec2= fs::File::create("./data_c2.csv").unwrap();
    let mut filephi= fs::File::create("./data_phi.csv").unwrap();
    
    for n in 0..nt{ 
          poisson::Poisson::compute(&mut fl,1,[-1.,-1.,0.,0.9]);
          poisson::Poisson::compute(&mut fl,2,[-1.,-1.,0.,0.1]);
          poisson::Poisson::compute(&mut al,1,[-1.,-1.,0.21,0.]);
          poisson::Poisson::compute(&mut al,2,[-1.,-1.,0.79,0.]);

          poisson::Poisson::compute(&mut fl,0,[-1.,-1.,-0.1,0.]);
          poisson::Poisson::compute(&mut el,0,[-1.,-1.,0.75,0.9]);
          poisson::Poisson::compute(&mut al,0,[-1.,-1.,0.7,0.75]);
          // laplace::Laplace::compute(&mut al,1);
          navier_stokes_channel::NavierStokesChannel::compute(&mut fch);
          navier_stokes_channel::NavierStokesChannel::compute(&mut ach);

          burgers::Burgers::compute(&mut fch,3,[0.8,0.5,-1.,-1.]);
          burgers::Burgers::compute(&mut fch,4,[0.1,0.05,-1.,-1.]);
          burgers::Burgers::compute(&mut ach,3,[0.21,0.1,-1.,-1.]);
          burgers::Burgers::compute(&mut ach,4,[0.7,0.79,-1.,-1.]);
     
     // let mut fg = Figure::new();
     // let domain = fch;
     // fg.axes3d().surface(&domain.data[0],domain.mesh_n[0],domain.mesh_n[0],
     //      Some((domain.coor[0],domain.coor[1],domain.coor[0]+domain.size[0],domain.coor[1]+domain.size[1]))
     //      ,&[]);
     // let domain = ach;
     // fg.axes3d().surface(&domain.data[0],domain.mesh_n[0],domain.mesh_n[0],
     //      Some((domain.coor[0],domain.coor[1],domain.coor[0]+domain.size[0],domain.coor[1]+domain.size[1]))
     //      ,&[]);
     // fg.show();
     // let domain = fl;
     // fg.axes3d().surface(&domain.data[1],domain.mesh_n[0],domain.mesh_n[0],
     //      Some((domain.coor[0],domain.coor[1],domain.coor[0]+domain.size[0],domain.coor[1]+domain.size[1]))
     //      ,&[]);
     // let domain = al;
     // fg.axes3d().surface(&domain.data[1],domain.mesh_n[0],domain.mesh_n[0],
     //      Some((domain.coor[0],domain.coor[1],domain.coor[0]+domain.size[0],domain.coor[1]+domain.size[1]))
     //      ,&[]);
     // fg.show();

     

          write_data(&ach.data[0], &mut fileu);
          for i in 0..3{
               write_data(&data_zero, &mut fileu);
          }
          write_data(&fch.data[0], &mut fileu);

          write_data(&ach.data[1],&mut filev);
          for i in 0..3{
               write_data(&data_zero,  &mut filev);
          }
          write_data(&fch.data[1], &mut filev);

          write_data(&ach.data[2], &mut filep);
          for i in 0..3{
               write_data(&data_zero, &mut filep);
          }
          write_data(&fch.data[2], &mut filep);

          write_data(&ach.data[3], &mut filec1);
          write_data(&al.data[0], &mut filec1);
          write_data(&data_zero, &mut filec1);
          write_data(&fl.data[0], &mut filec1);
          write_data(&fch.data[3], &mut filec1);

          write_data(&ach.data[4], &mut filec2);
          write_data(&al.data[1], &mut filec2);
          write_data(&data_zero, &mut filec2);
          write_data(&fl.data[1], &mut filec2);
          write_data(&fch.data[4], &mut filec2);

          write_data(&data_zero, &mut filephi);
          write_data(&al.data[2], &mut filephi);
          write_data(&el.data[0], &mut filephi);
          write_data(&fl.data[2], &mut filephi);
          write_data(&data_zero, &mut filephi);
    }


     println!("Complete!");
     println!("{:?}",now.elapsed());
     
}
fn write_data(data:&Array2<f64>,file:&mut fs::File){
     let ny = data.shape()[0];
     let nx = data.shape()[1];
     for i in 0..ny{
          for j in 0..nx{ 
               let s:String=format!("{:.5},",data[[i,j]]);
               file.write_all(s.as_bytes()).unwrap();
          }
          file.write_all(b"\n").unwrap();
     }
}