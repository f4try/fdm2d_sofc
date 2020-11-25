use fdm2d_sofc::*;

fn main() {
    // let 
    poisson::Poisson::compute();
    laplace::Laplace::compute();
    navier_stokes_channel::NavierStokesChannel::compute();
    println!("Complete!");
}
