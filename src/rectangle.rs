use ndarray::*;
use std::collections::HashMap;
pub struct Rectangle{
    pub coor:[f64;2],//x0,y0
    pub size:[f64;2],//width, height
    pub mesh_n:[usize;2],//nx,ny
    pub mesh_d:[f64;2],//dx,dy
    pub dt:f64,
    pub data_type:RectangleType,
    pub data:Vec<Array2<f64>>,
    pub params:Vec<f64>,
    // p:Array2<f64>,
    // v:Array2<f64>,
}
pub enum RectangleType{
    Channel, //data:u,v,p,c1,c2;params:rho,nu,F,D
    Electrode, //data:phis,c1,c2;params:D,sigma_s
    Electrolye, //data:phil;params:sigma_l
}

impl Rectangle{
    pub fn new(x0:f64,y0:f64,width:f64,height:f64
        ,nx:usize,ny:usize,dt:f64,data_type:RectangleType,params:Vec<f64>)->Self{
            let dx = width/nx as f64;
            let dy = height/ny as f64;
            let mut data:Vec<Array2<f64>> = vec![];
            // let mut params:Vec<f64> = vec![];
            let mut data_num = 0;
            // let mut param_num = 0;
            match data_type{
                RectangleType::Channel=>{data_num=5;},
                RectangleType::Electrode=>{data_num=3;},
                RectangleType::Electrolye=>{data_num=1;},
                _=>{},
            }
            for i in 0..data_num{
                 data.push(Array2::<f64>::zeros((ny,nx)));
            }
            // for i in 0..param_num{
            //      params.push(1.);
            // }
        Self{
            coor:[x0,y0],
            size:[width,height],
            mesh_n:[nx,ny],
            mesh_d:[dx,dy],
            dt:dt,
            data_type:data_type,
            data:data,
            params
        }
    }
}