use ndarray::*;
use std::collections::HashMap;
pub struct Rectangle{
    coor:[usize;2],//x0,y0
    size:[f64;2],//width, height
    mesh_n:[usize;2],//nx,ny
    mesh_d:[f64;2],//dx,dy
    type:RectangleType,
    data:HashMap<&str,Array2<f64>>,
    // p:Array2<f64>,
    // v:Array2<f64>,
}
pub enum RectangleType{
    Channel, //p,v,c1,c2
    Electrode, //phis,c1,c2
    Electrolye, //phil
}

impl Rectangle{
    pub fn new(x0:usize,y0:usize,width:f64,height:f64
        ,nx:usize,ny:usize,type:RectangleType)->Self{
            let dx = width/nx;
            let dy = height/ny;
            let data:Vec<Array2<f64>>;
            match type{
                RectangleType::Channel=>{
                    for
                },

            }
        Self{
            coor:[x0,y0],
            size:[width,height],
            mesh_n:[nx,ny],
            mesh_d:[dx,dy],
            type:type,
            data:data,
        }
    }
}