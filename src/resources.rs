use bevy::{prelude::*, math::vec2};

pub struct Path{
    pub id: String,
    pub points: Vec<Vec2>
}

impl Path {
    
    pub fn segment_len(&self) -> usize{
        self.points.len() - 1
    }

    pub fn segment(&self, n: usize) -> (Vec2, Vec2) {
        (self.points[n], self.points[n + 1])
    }
}

pub struct Route{
    pub id: String,
    pub paths: Vec<usize>
}
#[derive(Resource)]
pub struct RoutingTable{
    pub routes: Vec<Route>,
    pub paths: Vec<Path>,
}

impl Default for RoutingTable {
    fn default() -> Self {
        let mut paths = Vec::new();
        let mut routes = Vec::new();
        
        paths.push( Path { id: "p_1".to_owned(), points: vec![vec2(0., 300.), vec2(0., -50.), vec2(50., -150.), vec2(100., -200.0), vec2(200., -250.), vec2(300., -250.), vec2(400., -200.), vec2(450., -150.), vec2(500., -50.), vec2(500., 300.)] });

        routes.push( Route{ id:"r_1".to_owned(), paths: vec![0]});

        RoutingTable { paths, routes }
    }
}