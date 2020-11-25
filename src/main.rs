use fdm2d_sofc::*;
// use ndarray::*;

fn main() {
    let l_ch = 20.;//Length of channel and cell 
    let h_el = 2.;// Thickness of electrolyte
    let h_fl = 1.;// Thickness of fuel electrode layer
    let h_al = 1.;// Thickness of air electrode layer
    let h_fch = 3.;// Thickness of fuel channel
    let h_ach = 3.;// Thickness of air channel

    let dx = 1.;
    let dy = 0.2;


    poisson::Poisson::compute();
    laplace::Laplace::compute();
    navier_stokes_channel::NavierStokesChannel::compute();
    println!("Complete!");
}
