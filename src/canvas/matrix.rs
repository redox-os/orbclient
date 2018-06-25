use point::Point;

#[derive(Copy, Clone)]
pub struct Matrix {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }

    pub fn set_transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32){
        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;
        self.e = e;
        self.f = f;
    }


    pub fn transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32){
        let org_a= self.a;
        let org_b= self.b;
        let org_c= self.c;
        let org_d= self.d;
        let org_e= self.e;
        let org_f= self.f;

        self.a = org_a * a + org_c * b;
        self.b = org_b * a + org_d * b;
        self.c = org_a * c + org_c * d;
        self.d = org_b * c + org_d * d;
        self.e = org_a * e + org_c * f + org_e;
        self.f = org_b * e + org_d * f + org_f;
    }

    pub fn apply_to_point(&mut self, point: Point) -> (Point){
        Point::new(
            point.x * self.a + point.y * self.c + self.e,
            point.x * self.b + point.y * self.d + self.f
        )
    }
}
