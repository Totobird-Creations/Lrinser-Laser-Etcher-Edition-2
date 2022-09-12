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
    fn new_split(&mut self, right_half : bool, top_half : bool) -> RenderNode {
        let mut new = RenderNode::new();
        new.iteration = self.iteration + 1;
        new.position  = [
            self.position[0] + get_pixel_size(self.iteration + 1) * (right_half as i32 as f32),
            self.position[1] + get_pixel_size(self.iteration + 1) * (top_half   as i32 as f32)
        ];
        return new;
    }
    pub fn split(&mut self) {
        match (self.split) {
            RenderSplitOption::Wait => {
                self.split = RenderSplitOption::Continue(RenderSplit {
                    tl: Box::new(self.new_split(false, true)),
                    tr: Box::new(self.new_split(true, true)),
                    bl: Box::new(self.new_split(false, false)),
                    br: Box::new(self.new_split(true, false))
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
    pub fn check(&mut self) {
        match (self.split) {
            RenderSplitOption::Wait => {
                // TODO : Fix positioning.
                if (self.position[0] > 0.25) {
                    self.split = RenderSplitOption::Stop;
                }
            },
            RenderSplitOption::Stop => {},
            RenderSplitOption::Continue(ref mut split) => {
                split.bl.check();
                split.tl.check();
                split.br.check();
                split.tr.check();
            }
        };
    }
    pub fn get_pixel(&self, position : [f32; 2]) -> u8 {
        if (self.iteration == 3) {
            println!("{:?}", position);
        }
        return match (&self.split) {
            RenderSplitOption::Wait => 255,
            RenderSplitOption::Stop => 127,
            RenderSplitOption::Continue(ref split) => {
                let center_pos = [
                    self.position[0] + get_pixel_size(self.iteration) / 2.0,
                    self.position[1] + get_pixel_size(self.iteration) / 2.0
                ];
                if (position[0] < center_pos[0]) {
                    if (position[1] < center_pos[1]) {
                        split.bl.get_pixel(position)
                    } else {
                        split.tl.get_pixel([position[0], center_pos[1]])
                    }
                } else {
                    if (position[1] < center_pos[1]) {
                        split.br.get_pixel([center_pos[0], position[1]])
                    } else {
                        split.tr.get_pixel(center_pos)
                    }
                }
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
