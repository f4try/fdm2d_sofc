use ndarray::*;
use gnuplot::*;
use std::fs;
use std::io::Write;
use std::time::{Duration, SystemTime};
use crate::rectangle::*;
pub struct Burgers;
impl Burgers{
    // pub fn build_up_b(rho:f64,dt:f64,dx:f64,dy:f64,u:&Array2<f64>,v:&Array2<f64>)->Array2<f64>{
    //     let row = u.shape()[0];
    //     let col = u.shape()[1];
    //     let mut b = Array2::<f64>::zeros((row,col));
    //     for i in 1..row-1{
    //         for j in 1..col-1{
    //             b[[i,j]] = rho * dx.powi(2) * dy.powi(2)/2./(dx.powi(2)+dy.powi(2))
    //                         *(1./dt*(u[[i,j+1]]-u[[i,j-1]])/2./dx
    //                         +1./dt*(v[[i+1,j]]-v[[i-1,j]])/2./dy
    //                         -(u[[i,j+1]]-u[[i,j-1]])/2./dx*(u[[i,j+1]]-u[[i,j-1]])/2./dx
    //                         -2.*(u[[i+1,j]]-u[[i-1,j]])/2./dy*(v[[i,j+1]]-v[[i,j-1]])/2./dx
    //                         -(v[[i+1,j]]-v[[i-1,j]])/2./dy*(v[[i+1,j]]-v[[i-1,j]])/2./dy
    //                         );
    //         }
    //     }
    //     for i in 1..row-1{
    //         b[[i,col-1]] = rho * dx.powi(2) * dy.powi(2)/2./(dx.powi(2)+dy.powi(2))
    //                         *(1./dt*(u[[i,0]]-u[[i,col-2]])/2./dx
    //                         +1./dt*(v[[i+1,col-1]]-v[[i-1,col-1]])/2./dy
    //                         -(u[[i,0]]-u[[i,col-2]])/2./dx*(u[[i,0]]-u[[i,col-2]])/2./dx
    //                         -2.*(u[[i+1,col-1]]-u[[i-1,col-1]])/2./dy*(v[[i,0]]-v[[i,col-2]])/2./dx
    //                         -(v[[i+1,col-1]]-v[[i-1,col-1]])/2./dy*(v[[i+1,col-1]]-v[[i-1,col-1]])/2./dy
    //                         );
    //     }
    //     for i in 1..row-1{
    //         b[[i,0]] = rho * dx.powi(2) * dy.powi(2)/2./(dx.powi(2)+dy.powi(2))
    //                         *(1./dt*(u[[i,1]]-u[[i,col-1]])/2./dx
    //                         +1./dt*(v[[i+1,0]]-v[[i-1,0]])/2./dy
    //                         -(u[[i,1]]-u[[i,col-1]])/2./dx*(u[[i,1]]-u[[i,col-1]])/2./dx
    //                         -2.*(u[[i+1,0]]-u[[i-1,0]])/2./dy*(v[[i,1]]-v[[i,col-1]])/2./dx
    //                         -(v[[i+1,0]]-v[[i-1,0]])/2./dy*(v[[i+1,0]]-v[[i-1,0]])/2./dy
    //                         );
    //     }
    //     return b;
    // }

    // pub fn pressure_poisson_periodic(p:&Array2<f64>,dx:f64,dy:f64)->Array2<f64>{
    //     let mut p = p.clone();
    //     let row = p.shape()[0];
    //     let col = p.shape()[1];
    //     let mut pn = Array2::<f64>::zeros((row,col));
    //     let nit = 50;
    //     for q in 0..nit{
    //         pn = p.clone();
    //         for i in 1..row-1{
    //             for j in 1..col-1{
    //                 p[[i,j]] = ((pn[[i,j+1]]+pn[[i,j-1]])*dy.powi(2)
    //                             +(pn[[i+1,j]]+pn[[i-1,j]])*dx.powi(2))
    //                             /2./(dx.powi(2)+dy.powi(2));
    //             }
    //             p[[i,col-1]] = ((pn[[i,0]]+pn[[i,col-2]])*dy.powi(2)
    //                             +(pn[[i+1,col-1]]+pn[[i-1,col-1]])*dx.powi(2))
    //                             /2./(dx.powi(2)+dy.powi(2));
    //             p[[i,0]] = ((pn[[i,1]]+pn[[i,col-1]])*dy.powi(2)
    //                             +(pn[[i+1,0]]+pn[[i-1,0]])*dx.powi(2))
    //                             /2./(dx.powi(2)+dy.powi(2));
    //         }
    //         for j in 0..col{
    //             p[[row-1,j]] = p[[row-2,j]];
    //             p[[0,j]] = p[[1,j]];
    //         }
    //     }
    //     return p;
    // }

    pub fn compute(domain:&mut Rectangle,data_index:usize,boundary:[f64;4]){
        // let now = SystemTime::now();
        let nx = domain.mesh_n[0];
        let ny = domain.mesh_n[1];
        // let nt = 10;
        // let nit = 50;
        let c = 1;
        let dx = domain.mesh_d[0];
        let dy = domain.mesh_d[1];
        
        // let rho = domain.params[0];
        let nu = domain.params[0];
        // let F = domain.params[2];
        let dt = domain.dt;

        let mut u = domain.data[0].clone();
        let mut un = domain.data[0].clone();

        let mut v = domain.data[1].clone();
        let mut vn = domain.data[1].clone();

        // let mut p = domain.data[2].clone();
        // let mut pn = domain.data[2].clone();

        let mut c = domain.data[data_index].clone();
        let mut cn = domain.data[data_index].clone();
        
        // let mut b = Array2::<f64>::zeros((ny,nx));

        let mut udiff = 1.;
        // let mut stepcount = 0;

        // while udiff > 0.001{
        //     un = u.clone();
        //     vn = v.clone();

            // b = Self::build_up_b(rho, dt, dx, dy, &u, &v);
            // p = Self::pressure_poisson_periodic(&p, dx, dy);

            for i in 1..ny-1{
                for j in 1..nx-1{
                    c[[i,j]] = cn[[i,j]] 
                                - un[[i,j]]*dt/dx*(cn[[i,j]]-cn[[i,j-1]])
                                - vn[[i,j]]*dt/dy*(cn[[i,j]]-cn[[i-1,j]])
                                + nu*(dt/dx.powi(2)*(cn[[i,j+1]]-2.*cn[[i,j]]+cn[[i,j-1]])
                                    +dt/dy.powi(2)*(cn[[i+1,j]]-2.*cn[[i,j]]+cn[[i-1,j]])
                                    );
                }
                c[[i,0]] = boundary[0];
                c[[i,nx-1]] = boundary[1];
            }
            for j in 0..nx{
                c[[0,j]] = c[[1,j]];
                c[[ny-1,j]] = c[[ny-2,j]];
            }

            // stepcount+=1;
            // udiff = (u.sum()-un.sum())/u.sum();
        // }
        // println!("{:?}",now.elapsed());
        // println!("{:?}",udiff);
        // println!("{}",stepcount);
        


        // let mut fg = Figure::new();
        // fg.axes3d().surface(&u,nx,ny,Some((0.,0.,2.,2.)),&[]);
        // fg.show();
        // let mut fg = Figure::new();
        // fg.axes3d().surface(&v,nx,ny,Some((0.,0.,2.,2.)),&[]);
        // fg.show();
        // let mut fg = Figure::new();
        // fg.axes3d().surface(&p,nx,ny,Some((0.,0.,2.,2.)),&[]);
        // fg.show();
        // let mut fileu = fs::File::create("/home/zz/work/rust/cfd_tut/tut15_u.csv").unwrap();
        // let mut filev = fs::File::create("/home/zz/work/rust/cfd_tut/tut15_v.csv").unwrap();
        // let mut filep = fs::File::create("/home/zz/work/rust/cfd_tut/tut15_p.csv").unwrap();

        // for i in 0..ny{
        //     for j in 0..nx{
        //         // println!("{},{},{},{},{}",i as f64*dx,j as f64*dy
        //         //     ,u[[i,j]],v[[i,j]],p[[i,j]]);
        //         let su:String=format!("{},",u[[i,j]]);
        //         fileu.write_all(su.as_bytes()).unwrap();
        //         let sv:String=format!("{},",v[[i,j]]);
        //         filev.write_all(sv.as_bytes()).unwrap();
        //         let sp:String=format!("{},",p[[i,j]]);
        //         filep.write_all(sp.as_bytes()).unwrap();
        //     }
        //     fileu.write_all(b"\n").unwrap();
        //     filev.write_all(b"\n").unwrap();
        //     filep.write_all(b"\n").unwrap();
        // }
        // domain.data[0] = u;
        // domain.data[1] = v;
        domain.data[data_index] = c;
    }
}

