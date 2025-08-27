use std::{
    error::Error,
    io::{Read, Write},
};

use state::Cord;

#[derive(Debug, Default)]
pub struct History {
    cords: Vec<Cord>,
    step: usize,
}

impl History {
    pub fn new(cord: Cord) -> Self {
        Self {
            cords: vec![cord],
            step: 0,
        }
    }

    pub fn reset(&mut self, cord: Cord) {
        self.cords = vec![cord];
        self.step = 0;
    }

    pub fn step(&self) -> usize {
        self.step
    }

    pub fn push(&mut self, cord: Cord) {
        self.step += 1;
        if self.step == self.cords.len() {
            self.cords.push(cord);
        } else {
            self.cords[self.step] = cord;
            self.cords.truncate(self.step + 1);
        }
    }

    pub fn undo(&mut self) -> Option<&Cord> {
        if self.step == 0 {
            None
        } else {
            self.step -= 1;
            Some(&self.cords[self.step])
        }
    }

    pub fn undo_all(&mut self) -> &Cord {
        self.step = 0;
        &self.cords[0]
    }

    pub fn redo(&mut self) -> Option<&Cord> {
        if self.step + 1 == self.cords.len() {
            None
        } else {
            self.step += 1;
            Some(&self.cords[self.step])
        }
    }

    pub fn save<W: Write>(&self, bw: &mut W) -> Result<(), Box<dyn Error>> {
        bw.write_all(&(self.cords.len() as u32).to_le_bytes())?;
        bw.write_all(&(self.step as u32).to_le_bytes())?;
        for cord in self.cords.iter() {
            for x in cord {
                bw.write_all(&x.to_le_bytes())?;
            }
        }
        Ok(())
    }

    // Load function: n_beakers is received as a parameter
    pub fn load<R: Read>(&mut self, br: &mut R, n_beakers: usize) -> Result<(), Box<dyn Error>> {
        let mut buf = [0; 4];

        // Read `n_coords`
        br.read_exact(&mut buf)?;
        let n_cords = u32::from_le_bytes(buf) as usize;

        // Read `pos`
        br.read_exact(&mut buf)?;
        self.step = u32::from_le_bytes(buf) as usize;

        // Initialize `cords` with the correct dimensions
        self.cords = Vec::with_capacity(n_cords);
        for _ in 0..n_cords {
            // Create each inner `cord` with the constant `n_beakers` size
            let mut current_cord = Cord::with_capacity(n_beakers);
            for _ in 0..n_beakers {
                // Read n_beakers elements
                br.read_exact(&mut buf)?;
                current_cord.push(u32::from_le_bytes(buf));
            }
            self.cords.push(current_cord);
        }
        Ok(())
    }
}
