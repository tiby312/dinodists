
use axgeom::Vec2;
use axgeom::vec2;

#[derive(Clone)]
pub struct Spiral{
    point:[f32;2],
    rad:f32,
    start:f32,
    rate:f32,
    width:f32
}

pub struct SpiralInt(Spiral);
impl Iterator for SpiralInt{
    type Item=Vec2<isize>;
    fn next(&mut self)->Option<Vec2<isize>>{
        self.0.next().map(|a|vec2(a.x as isize,a.y as isize))
    }
}
impl std::iter::FusedIterator for SpiralInt{}

pub struct SpiralF64(Spiral);
impl Iterator for SpiralF64{
    type Item=Vec2<f64>;
    fn next(&mut self)->Option<Vec2<f64>>{
        self.0.next().map(|a|vec2(a.x as f64,a.y as f64))
    }
}
impl std::iter::FusedIterator for SpiralF64{}



impl Spiral{
    pub fn new(point:[f32;2],circular_grow:f32,outward_grow:f32)->Spiral{
        Spiral{point,rad:0.0,start:1.0,rate:outward_grow,width:circular_grow}
    }
    pub fn get_circular_grow(&self)->f32{
        self.width
    }
    pub fn get_outward_grow(&self)->f32{
        self.rate
    }
    pub fn as_isize(self)->SpiralInt{
        SpiralInt(self)
    }
    pub fn as_f64(self)->SpiralF64{
        SpiralF64(self)
    }
}

impl std::iter::FusedIterator for Spiral{}

impl Iterator for Spiral{
    type Item=Vec2<f32>;
    fn next(&mut self)->Option<Vec2<f32>>{
        
        let length=self.start+self.rate*self.rad;

        let x=self.point[0]+self.rad.cos()*length;
        let y=self.point[1]+self.rad.sin()*length;

        self.rad+=self.width/length;

        Some(vec2(x,y))

    }
}

use crate::Dist2;

impl Dist2<f32> for Spiral{}
impl Dist2<f64> for SpiralF64{}
impl Dist2<isize> for SpiralInt{}