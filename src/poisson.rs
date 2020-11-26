use ndarray::*;
use gnuplot::*;
use crate::rectangle::*;

pub struct Poisson;
impl Poisson{
    pub fn plot2D(p:&Array2<f64>,nx:usize,ny:usize,x:f64,y:f64){
        let mut fg = Figure::new();
        fg.axes3d().surface(p,nx,ny,Some((0.,0.,x,y)),&[]);
        fg.show();
    }

    pub fn abs(array:&Array2<f64>)->Array2<f64>{
        let mut result = array.clone();
        Zip::from(&mut result).apply(|v|*v=v.abs());
        result
    }

    pub fn poisson2d(p:&mut Array2<f64>,b:&Array2<f64>,dx:f64,dy:f64,l1norm_target:f64){
        let mut l1norm =1.;
        let row = p.shape()[0];
        let col = p.shape()[1];
        // let mut pn = Array2::zeros((p.shape()[0],p.shape()[1]));
        let mut pn = p.clone();
        // while l1norm>l1norm_target{
            pn=p.clone();
        // for _ in 0..nt{
            Zip::from(p.slice_mut(s![1..row-1,1..col-1]))
                .and(pn.slice(s![2..row,1..col-1]))
                .and(pn.slice(s![0..row-2,1..col-1]))
                .and(pn.slice(s![1..row-1,2..col]))
                .and(pn.slice(s![1..row-1,0..col-2]))
                .and(b.slice(s![1..row-1,1..col-1]))
                .apply(|pij,&pip1j,&pim1j,&pijp1,&pijm1,&bij|
                    *pij=(dy.powi(2)*(pijp1+pijm1)+dx.powi(2)*(pip1j+pim1j)-dx.powi(2)*dy.powi(2)*bij)
                    /2./(dx.powi(2)+dy.powi(2)));
            // let mut l1norm_array = Array2::zeros((p.shape()[0],p.shape()[1]));
            // Zip::from(&mut l1norm_array)
            //     .and(&*p)
            //     .and(&pn)
            //     .apply(|vl:f64, vp:f64, vpn:f64|*vl=vp.abs()-vpn.abs());
            // l1norm = l1norm_array.sum();
            // Zip::from(p.slice_mut(s![0,..])).apply(  |pb|*pb=0.);
            // Zip::from(p.slice_mut(s![..,col-1])).apply(|pb|*pb=0.);
            for i in 0..row{
                p[[i,0]] = p[[i,1]];
                p[[i,col-1]] = p[[i,col-2]];
            }
            // l1norm= ((Self::abs(&p)-Self::abs(&pn)).sum()/pn.sum()).abs();
            // println!("l1norm:{:?}",l1norm);
        // }
    }

    pub fn compute(domain:&mut Rectangle,data_index:usize,boundary:[f64;4]){
        let nx = domain.mesh_n[0];
        let ny = domain.mesh_n[1];
        // let nt = 100;
        let dx = domain.mesh_d[0];
        let dy = domain.mesh_d[1];
        let xmax = domain.coor[0]+domain.size[0];
        let ymax = domain.coor[1]+domain.size[1];
        let mut p=domain.data[data_index].clone();
        let pd=domain.data[data_index].clone();
        let mut b=Array2::<f64>::zeros((ny,nx));
        // Zip::from(b.slice_mut(s![(nx/4) as usize,(ny/4) as usize])).apply(|pb|*pb=100.);
        // Zip::from(b.slice_mut(s![(3*nx/4) as usize,(3*ny/4) as usize])).apply(|pb|*pb=-100.);
        // Self::plot2D(&b, nx, ny, xmax, ymax);
        let row = p.shape()[0];
        let col = p.shape()[1];
        if boundary[0]>0.{
            Zip::from(p.slice_mut(s![..,0])).apply(|pb|*pb=boundary[0]);
        }
        if boundary[1]>0.{
            Zip::from(p.slice_mut(s![..,col-1])).apply(|pb|*pb=boundary[1]);
        }
        if boundary[2]>0.{
            Zip::from(p.slice_mut(s![0,..])).apply(|pb|*pb=boundary[2]);
        }
        if boundary[3]>0.{
            Zip::from(p.slice_mut(s![row-1,..])).apply(|pb|*pb=boundary[3]);
        }
        // Self::plot2D(&p, nx, ny, xmax, ymax);
        Self::poisson2d(&mut p,&b, dx, dy, 1e-6);
        // Self::plot2D(&p, nx, ny, xmax, ymax);
        domain.data[data_index] = p;

    }
}
