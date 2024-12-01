#[derive(Debug, Clone)]
pub struct VectorField2D {
    pub width: usize,
    pub height: usize,
    pub field: Vec<Vec<[f32; 2]>>,
}

#[derive(Debug, Clone)]
pub struct ColorField2D {
    pub width: usize,
    pub height: usize,
    pub field: Vec<Vec<f32>>,
}

impl ColorField2D {
    pub fn new(width: usize, height: usize, initial_value: f32) -> Self {
        let field = vec![vec![initial_value; width]; height];
        Self {
            width,
            height,
            field,
        }
    }

    pub fn bilinear_interpolation(&self, x: f32, y: f32) -> f32 {
        let x0 = x.floor() as isize;
        let x1 = x0 + 1;
        let y0 = y.floor() as isize;
        let y1 = y0 + 1;

        // Garantir que os índices estejam dentro dos limites
        let clamp = |v: isize, min: isize, max: isize| v.max(min).min(max) as usize;

        let x0 = clamp(x0, 0, self.width as isize - 1);
        let x1 = clamp(x1, 0, self.width as isize - 1);
        let y0 = clamp(y0, 0, self.height as isize - 1);
        let y1 = clamp(y1, 0, self.height as isize - 1);

        let q00 = self.field[y0][x0];
        let q01 = self.field[y0][x1];
        let q10 = self.field[y1][x0];
        let q11 = self.field[y1][x1];

        let tx = x - x0 as f32;
        let ty = y - y0 as f32;

        let a = q00 * (1.0 - tx) + q01 * tx;
        let b = q10 * (1.0 - tx) + q11 * tx;

        a * (1.0 - ty) + b * ty
    }

    pub fn update(&mut self, velocity_field: &VectorField2D, delta_time: f32) -> Self {
        let mut new_field = self.field.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let velocity = velocity_field.field[y][x];
                let px = x as f32 - velocity[0] * delta_time;
                let py = y as f32 - velocity[1] * delta_time;

                new_field[y][x] = self.bilinear_interpolation(px, py);
            }
        }

        Self {
            width: self.width,
            height: self.height,
            field: new_field,
        }
    }
}

impl VectorField2D {

    pub fn onMouseClick(&self, x: i16, y: i16, deltaX: i16, deltaY: i16) {
        // É necessário ver como chamar e como passar referencia de field.
        /*
        let magnitude = 10.0;
        let screenRatioX = 40.0 / 800.0;
        let screenRatioY = 40.0 / 600.0;

        let xPos = (x as f32 * screenRatioX).round();
        let yPos = (y as f32 * screenRatioY).round();

        for (i, row) in self.field.iter_mut().enumerate() {
            for (j, value) in row.iter_mut().enumerate() {
                let cellX = i as f32;
                let cellY = j as f32;
                let distance = ((xPos - cellX).powi(2) + (yPos - cellY).powi(2)).sqrt();

                let influence = 1.0 / (distance + 1.0);

                let deltaMagnitudeX = (deltaX as f32) * influence * magnitude;
                let deltaMagnitudeY = (deltaY as f32) * influence * magnitude;

                let x_val = value[0];
                let y_val = value[1];
                let new_x_val = x_val + deltaMagnitudeX;
                let new_y_val = y_val + deltaMagnitudeY;

                // Usando o método set para alterar o valor
                if let Err(e) = self.set(i, j, [new_x_val, new_y_val]) {
                    println!("Erro ao definir valor: {}", e);
                }

                println!("Nova posição: ({}, {}), valor alterado: ({}, {})", i, j, new_x_val, new_y_val);
            }
        }
        */
    }


}