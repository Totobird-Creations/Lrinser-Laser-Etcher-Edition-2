use crate::parse::node::EvaluatedValues;
use super::settings::RenderSettings;


#[derive(Clone, Debug)]
pub struct RenderNode {
    split      : RenderSplitOption,
    iteration  : u32,
    position   : [f32; 2],
    iterations : u32
}
impl RenderNode {
    pub fn new(iterations : u32) -> RenderNode {
        return RenderNode {
            split      : RenderSplitOption::Wait,
            iteration  : 0,
            position   : [0.0, 0.0],
            iterations : iterations
        };
    }
    fn new_split(&self, offset_mult : [f32; 2]) -> RenderNode {
        let mut new = RenderNode::new(self.iterations);
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
    pub fn check(&mut self, settings : &RenderSettings, column_values : &Vec<EvaluatedValues>) {
        match (self.split) {
            RenderSplitOption::Wait => {
                let left_index   = (self.position[0] * (column_values.len() as f32 - 1.0)) as usize;
                let right_index  = left_index + 1;
                // Get Y coord at top and bottom of split.
                let bottom_value = settings.frame[1] + (settings.frame[3] - settings.frame[1]) * (self.position[1] as f64);
                let top_value    = settings.frame[1] + (settings.frame[3] - settings.frame[1]) * ((self.position[1] + get_pixel_size(self.iteration)) as f64);
                // Collect all values on the left and right edge.
                let mut values = column_values[left_index].get_values().clone();
                values.append(&mut column_values[right_index].get_values().clone());
                let mut passed = false;
                for value in values {
                    if (value >= bottom_value && value < top_value) {
                        passed = true;
                        break;
                    }
                }
                if (! passed) {
                    self.split = RenderSplitOption::Stop;
                }
            },
            RenderSplitOption::Stop => (),
            RenderSplitOption::Continue(ref mut split) => {
                split.bl.check(settings, column_values);
                split.tl.check(settings, column_values);
                split.br.check(settings, column_values);
                split.tr.check(settings, column_values);
            }
        };
    }
    fn _check_top(&self, _settings : &RenderSettings) {
        if (! matches!(self.split, RenderSplitOption::Wait)) {panic!("`check_top` called when split option is not `RenderSplitOption::Wait`.")};
        panic!("Unimplemented.");
    }
    fn _check_side(&self, settings : &RenderSettings, _y_values : &EvaluatedValues) {
        if (! matches!(self.split, RenderSplitOption::Wait)) {panic!("`check_side` called when split option is not RenderSplitOption::Wait.")};
        let _bottom_value = settings.frame[1] + (settings.frame[3] - settings.frame[1]) * (self.position[1] as f64);
        let _top_value    = settings.frame[1] + (settings.frame[3] - settings.frame[1]) * ((self.position[1] + get_pixel_size(self.iteration)) as f64);
        panic!("Unimplemented.");
    }
    pub fn get_pixel(&self, position : [f32; 2]) -> [u8; 3] {
        return match (&self.split) {
            RenderSplitOption::Wait => [255, 0, 0],
            RenderSplitOption::Stop => {
                let i = ((self.iteration as f64 / self.iterations as f64) * 255.0) as u8;
                [i, i, i]
            },
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
