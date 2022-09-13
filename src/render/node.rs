use crate::parse::node::EvaluatedValues;


#[derive(Clone, Debug)]
pub struct RenderNode {
    split     : RenderSplitOption,
    iteration : u32,
    position  : [f32; 2]
}
impl RenderNode {
    pub fn new() -> RenderNode {
        return RenderNode {
            split     : RenderSplitOption::Wait,
            iteration : 0,
            position  : [0.0, 0.0]
        };
    }
    fn new_split(&self, offset_mult : [f32; 2]) -> RenderNode {
        let mut new = RenderNode::new();
        new.iteration = self.iteration + 1;
        new.position  = [
            self.position[0] + get_pixel_size(self.iteration) * offset_mult[0],
            self.position[1] + get_pixel_size(self.iteration) * offset_mult[1]
        ];
        return new;
    }
    pub fn split(&mut self) {
        match (self.split) {
            RenderSplitOption::Wait => {
                self.split = RenderSplitOption::Continue(RenderSplit {
                    tl: Box::new(self.new_split([0.5, 0.0])),
                    tr: Box::new(self.new_split([0.5, 0.5])),
                    bl: Box::new(self.new_split([0.0, 0.0])),
                    br: Box::new(self.new_split([0.0, 0.5]))
                });
            }
            RenderSplitOption::Stop => (),
            RenderSplitOption::Continue(ref mut split) => {
                split.tl.split();
                split.tr.split();
                split.bl.split();
                split.br.split();
            }
        };
    }
    pub fn check(&mut self, column_values : &Vec<EvaluatedValues>) {
        match (self.split) {
            RenderSplitOption::Wait => {
                let left_index  = (self.position[0] * (column_values.len() - 1) as f32) as usize;
                let left_values = column_values[left_index].get_values();
                println!("{:?}", left_values);
                let mut passed = false;
                for i in 0..left_values.len() {
                    if (left_values[i] <= -1.5) {
                        passed = true;
                        break;
                    }
                }
                if (! passed) {
                    self.split = RenderSplitOption::Stop;
                }
                
            },
            RenderSplitOption::Stop => {},
            RenderSplitOption::Continue(ref mut split) => {
                split.bl.check(column_values);
                split.tl.check(column_values);
                split.br.check(column_values);
                split.tr.check(column_values);
            }
        };
    }
    pub fn get_pixel(&self, position : [f32; 2]) -> [u8; 3] {
        return match (&self.split) {
            RenderSplitOption::Wait => [(self.position[0] * 255.0) as u8, (self.position[1] * 255.0) as u8, 0],
            RenderSplitOption::Stop => [255, 255, 255],
            RenderSplitOption::Continue(ref split) => {
                let center_pos = [
                    self.position[0] + get_pixel_size(self.iteration + 1),
                    self.position[1] + get_pixel_size(self.iteration + 1)
                ];
                [split.bl.clone(), split.tl.clone(), split.br.clone(), split.tr.clone()][
                    if (position[0] < center_pos[0]) {0} else {1} +
                    if (position[1] < center_pos[1]) {0} else {2}
                ].get_pixel(position)
            }
        };
    }
}

fn get_pixel_size(iteration : u32) -> f32 {
    return 0.5_f32.powi(iteration as i32);
}


#[derive(Clone, Debug)]
pub enum RenderSplitOption {
    Wait,                 // Split if possible.
    Stop,                 // Do not split.
    Continue(RenderSplit) // Has already split.
}


#[derive(Clone, Debug)]
pub struct RenderSplit {
    tl : Box<RenderNode>,
    tr : Box<RenderNode>,
    bl : Box<RenderNode>,
    br : Box<RenderNode>
}
