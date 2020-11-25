use ndarray::*;
use std::collections::HashMap;
pub struct Rectangle{
    coor:[usize;2],//x0,y0
    size:[f64;2],//width, height
    mesh_n:[usize;2],//nx,ny
    mesh_d:[f64;2],//dx,dy
    data_type:RectangleType,
    data:Vec<Array2<f64>>,
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
        ,nx:usize,ny:usize,data_type:RectangleType)->Self{
            let dx = width/nx as f64;
            let dy = height/ny as f64;
            let mut data:Vec<Array2<f64>>;
            let mut data_num = 0;
            match data_type{
                RectangleType::Channel=>{data_num=4;},
                RectangleType::Electrode=>{data_num=2;},
                RectangleType::Electrolye=>{data_num=1;},
                _=>{},
            }
            for i in 0..data_num{
                 data.push(Array2::<f64>::zeros((nx,ny)));
            }
        Self{
            coor:[x0,y0],
            size:[width,height],
            mesh_n:[nx,ny],
            mesh_d:[dx,dy],
            data_type:data_type,
            data:data,
        }
    }
}