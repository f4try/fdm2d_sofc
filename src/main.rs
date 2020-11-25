use fdm2d_sofc::*;
// use ndarray::*;

fn main() {
    let l_ch = 20.;//Length of channel and cell 
    let h_el = 2.;// Thickness of electrolyte
    let h_fl = 1.;// Thickness of fuel electrode layer
    let h_al = 1.;// Thickness of air electrode layer
    let h_fch = 3.;// Thickness of fuel channel
    let h_ach = 3.;// Thickness of air channel

    let dx = 2.;
    let dy = 0.2;
    let nx = (l_ch/dx) as usize;
    let ny = 5;

    let ach = rectangle::Rectangle::new(0., 0.,
         l_ch, h_ach, nx, ny, rectangle::RectangleType::Channel);
    let al = rectangle::Rectangle::new(0., h_ach,
         l_ch, h_al, nx, ny, rectangle::RectangleType::Electrode);
    let el = rectangle::Rectangle::new(0., h_ach+h_al,
         l_ch, h_el, nx, ny, rectangle::RectangleType::Electrolye);
    let fl = rectangle::Rectangle::new(0., h_ach+h_al+h_el,
         l_ch, h_fl, nx, ny, rectangle::RectangleType::Electrode);
    let fch = rectangle::Rectangle::new(0., h_ach+h_al+h_el+h_fl,
         l_ch, h_fch, nx, ny, rectangle::RectangleType::Channel);
    poisson::Poisson::compute();
    laplace::Laplace::compute();
    navier_stokes_channel::NavierStokesChannel::compute();
    println!("Complete!");
}
