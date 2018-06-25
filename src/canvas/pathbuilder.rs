use alloc::Vec;
use edge::Edge;
use point::Point;

/// graphic path with similar functions like html canvas
#[derive(Clone)]
pub struct PathBuilder {
    last_point: Point,
    last_moved_point: Point,
    pub edges: Vec<Edge>,
}

impl PathBuilder {

    pub fn new() -> Self {
        PathBuilder {
            last_point : Point::new(0.0,0.0),
            last_moved_point : Point::new(0.0,0.0),
            edges: Vec::new(),
        }
    }

    /// move to position
    pub fn move_to(&mut self, x: f32, y: f32){
        //self.points.push((x,y,PathPointType::Move));
        self.last_point = Point::new(x,y);
        self.last_moved_point = Point::new(x,y);
    }

    /// create a line between the last and new point
    pub fn line_to(&mut self, x: f32, y: f32) {
        //self.points.push((x,y,PathPointType::Connect));
        self.edges.push(Edge::new(self.last_point, Point::new(x,y)));
        self.last_point = Point::new(x,y);
    }

    pub fn close_path(&mut self) {



        self.edges.push(Edge::new(self.last_point, self.last_moved_point));
        //self.x = x;
        //self.y = y;
    }


    /// quadratic bezier curve
    pub fn quadratic_curve_to(&mut self, argx1: f32, argy1: f32, argx2: f32, argy2: f32){
        let mut t:f32 = 0.0;
        let mut u:f32;
        let mut tt:f32;
        let mut uu:f32;
        let mut x:f32;
        let mut y:f32;


        let mut tmp_point = self.last_point;
        while t < 1.0 {
            u = 1.0 - t;
            uu = u * u;
            tt = t * t;

            x = self.last_point.x * uu;
            y = self.last_point.y * uu;

            x += 2.0 * u * t * argx1;
            y += 2.0 * u * t * argy1 ;

            x += tt * argx2;
            y += tt * argy2;

            t += 0.01;
            //self.points.push((x as f32, y as f32, PathPointType::Connect));
            self.edges.push(Edge::new(tmp_point, Point::new(x,y)));
            tmp_point = Point::new(x,y);
        }

        self.last_point = Point::new(argx2,argy2);
    }

    /// cubic bezier curve
    pub fn bezier_curve_to(&mut self, argx1: f32, argy1: f32, argx2: f32, argy2: f32, argx3: f32, argy3: f32){
        let mut t:f32 = 0.0;
        let mut u:f32;
        let mut tt:f32;
        let mut uu:f32;
        let mut uuu:f32;
        let mut ttt:f32;
        let mut x:f32;
        let mut y:f32;

        let mut tmp_point = self.last_point;
        while t < 1.0 {
            u = 1.0 - t;
            tt = t * t;
            uu = u * u;
            uuu = uu * u;
            ttt = tt * t;

            x = self.last_point.x as f32 * uuu;
            y = self.last_point.y as f32 * uuu;

            x += 3.0 * uu * t * argx1 as f32;
            y += 3.0 * uu * t * argy1 as f32;

            x += 3.0 * u * tt * argx2 as f32;
            y += 3.0 * u * tt * argy2 as f32;

            x += ttt * argx3 as f32;
            y += ttt * argy3 as f32;

            t += 0.01;
            //self.points.push((x as f32, y as f32, PathPointType::Connect));
            self.edges.push(Edge::new(tmp_point, Point::new(x as f32,y as f32)));
            tmp_point = Point::new(x as f32,y as f32);
        }

        self.last_point = Point::new(argx3,argy3);
    }
}
