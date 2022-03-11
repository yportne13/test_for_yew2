use super::settings::Settings;
use yew::{html, Html};

/////////////////////////
struct RNG {
    seed: i64,
}

impl RNG {
    fn next_int(&self) -> (i32,RNG) {
        let new_seed = (self.seed * 0x5DEECE66D + 0xB) & 0xFFFFFFFFFFFF;
        let next_rng = RNG{seed:new_seed};
        let n = (new_seed >> 16) as i32;
        (n,next_rng)
    }
    fn non_negative_small_int(&self) -> (i32,RNG) {
        let (i,r) = self.next_int();
        let non_neg = if i<0 {-(i+1)} else {i};
        let o = non_neg%16;
        (o,r)
    }
}
/////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub struct Signals {
    data: Vec<(i32,i32)>,
    pub show: Vec<(i32,i32)>,
    offset: i32,
    pub color: i32,
}

impl Signals {
    pub fn new_random(settings: &Settings, offset: i32, color: i32) -> Self {
        let mut rng = RNG{seed:(offset as i64)};

        let max_edge = 1000;
        let mut now_edge = 0;
        let edge = (0 .. 2*max_edge).map(|idx| {
            let (data,new_rng) = rng.non_negative_small_int();
            let ret = now_edge+(if idx%2==1 {data} else {0i32});
            now_edge = ret;
            rng = new_rng;
            ret
        });
        let mut is_high = 3;
        let data: Vec<(i32,i32)> = edge.map(|e| {
            is_high = (is_high+1)%4;
            (if is_high<2 {offset+15} else {offset},e)
        }).collect();
        let show = (&data).into_iter()
                .map(|&(x,y)| (x,y*settings.size-settings.x_axis))
                .collect();
        
        Self {
            data,
            show,
            offset,
            color,
        }
    }

    pub fn update(&mut self, settings: &Settings) {
        self.show = (&self.data).into_iter()
            .map(|&(x,y)| (x,y*settings.size-settings.x_axis))
            .collect();
    }

    pub fn render(&self) -> Html {
        let color = if self.color == 0 {"red"} else if self.color == 1 {"green"} else {"blue"};

        let mut points = String::new();
        for d in &self.show {
            points.push_str(&format!("{:.2},{:.2} ", d.1, d.0));
        }

        html! { <polyline points={points} fill="none" stroke={color} /> }
    }
}
