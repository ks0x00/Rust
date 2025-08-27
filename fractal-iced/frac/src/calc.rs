use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    sync::{mpsc::channel, Arc, Mutex},
    thread,
    time::Duration,
};

use base::{Pair, DATA_DIR, END_OF_LINE, F, INDEX_FILE};
use func::complex::Complex;
use iced::{widget::image::Handle, Point, Vector};
use image::{codecs::png::PngEncoder, ImageEncoder};
use param::{NParameter, SParameter};
use threadpool::ThreadPool;

use crate::{color::color_list, iterator, Iterator};

pub struct Calculator {
    param_text: String,
    nparam: NParameter,
    color_list: Vec<[u8; 4]>,
    image_size: Pair<usize>,
    image_building: Arc<Mutex<bool>>,
    pixel_iters: Arc<Mutex<Vec<Vec<usize>>>>,
    elapsed: Arc<Mutex<String>>,
}

impl Calculator {
    pub fn new(sparam: &SParameter, elapsed: Arc<Mutex<String>>) -> Self {
        let nparam = NParameter::from(sparam);
        let color_list = color_list(&nparam);
        let image_size = Pair {
            x: nparam.image_size_x,
            y: nparam.image_size_y,
        };
        let pixel_iters = Arc::new(Mutex::new(vec![vec![0; image_size.x]; image_size.y]));
        let mut calc = Self {
            param_text: sparam.to_text(),
            nparam,
            color_list,
            image_size,
            image_building: Arc::new(Mutex::new(false)),
            pixel_iters,
            elapsed,
        };
        calc.build_iters_channel_line();
        calc
    }

    pub fn image_size(&self) -> &Pair<usize> {
        &self.image_size
    }

    pub fn pixel_coord(&self, pixel: &Point) -> Pair<F> {
        self.nparam.pixel_coord(pixel)
    }

    pub fn vector_coord(&self, v: &Vector) -> Pair<F> {
        self.nparam.vector_coord(v)
    }

    pub fn pixel_color(&self, pixel: &Point) -> &[u8; 4] {
        &self.color_list[self.pixel_iters.lock().unwrap()[pixel.y as usize][pixel.x as usize]]
    }

    pub fn pixel_iter(&self, pixel: &Point) -> usize {
        self.pixel_iters.lock().unwrap()[pixel.y as usize][pixel.x as usize]
    }

    pub fn center_to(&self, pixel: &Point, sparam: &mut SParameter) {
        self.nparam.center_to(pixel, sparam);
    }

    pub fn translate(&self, v: &Vector, sparam: &mut SParameter) {
        self.nparam.translate(v, sparam);
    }

    pub fn expand_at_center(&self, factor: F, sparam: &mut SParameter) {
        self.nparam.expand_at_center(factor, sparam)
    }

    pub fn expand_at_point(&self, pixel: &Point, factor: F, sparam: &mut SParameter) {
        self.nparam.expand_at_point(pixel, factor, sparam)
    }

    pub fn center_and_expand(&self, pixel: &Point, factor: F, sparam: &mut SParameter) {
        self.nparam.center_and_expand(pixel, factor, sparam);
    }

    pub fn png(&self) {
        let mut data = vec![0_u8; 4 * self.image_size.x * self.image_size.y];
        let pixel_iters = self.pixel_iters.lock().unwrap().clone();
        let mut pos = 0;
        for i in 0..self.image_size.y {
            for j in 0..self.image_size.x {
                data[pos..pos + 4].copy_from_slice(&self.color_list[pixel_iters[i][j]]);
                pos += 4;
            }
        }

        let now = chrono::offset::Local::now();
        let suffix = format!(
            "{}_{}.png",
            &self.nparam.fractal_type.to_string()[..1],
            &now.format("%Y%m%d_%H%M%S").to_string()[2..]
        );
        let mut index_file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(format!("{}/{}", DATA_DIR, INDEX_FILE))
            .unwrap();
        if index_file.seek(SeekFrom::End(-1)).is_ok() {
            let mut s = String::new();
            index_file.read_to_string(&mut s).unwrap();
            if s != "\n" {
                index_file.write_all(END_OF_LINE.as_bytes()).unwrap();
            }
        }
        index_file.write_all(suffix.as_bytes()).unwrap();
        index_file.write_all(END_OF_LINE.as_bytes()).unwrap();
        index_file.write_all(self.param_text.as_bytes()).unwrap();
        index_file.write_all(END_OF_LINE.as_bytes()).unwrap();

        let filename = format!("{}/{}", DATA_DIR, suffix);
        let png_file = File::create(filename).unwrap();
        let encoder = PngEncoder::new(png_file);
        encoder
            .write_image(
                &data,
                self.image_size.x as u32,
                self.image_size.y as u32,
                image::ColorType::Rgba8,
            )
            .unwrap();
    }

    pub fn stop_building(&mut self) {
        *self.image_building.lock().unwrap() = false;
    }

    pub fn reset(&mut self, sparam: &SParameter) {
        if !sparam.check() {
            return;
        }
        self.param_text = sparam.to_text();
        let nparam = NParameter::from(sparam);
        // println!("{}", self.nparam.equal_color_list(&nparam));
        if !self.nparam.equal_color_list(&nparam) {
            self.color_list = color_list(&nparam);
            self.nparam.color_type = nparam.color_type;
            self.nparam.color_var = nparam.color_var;
        }
        if !self.nparam.equal_iters(&nparam) {
            self.stop_building();
            self.image_size.x = nparam.image_size_x;
            self.image_size.y = nparam.image_size_y;
            self.pixel_iters = Arc::new(Mutex::new(vec![
                vec![0; self.image_size.x];
                self.image_size.y
            ]));
            self.nparam = nparam;
            self.build_iters_channel_line();
        }
    }

    pub fn image_handle(&self) -> Handle {
        let pixel_iters = self.pixel_iters.lock().unwrap().clone();
        let mut pixels: Vec<u8> = vec![255; 4 * self.image_size.x * self.image_size.y];
        let mut pos = 0;
        for i in 0..self.image_size.y {
            for j in 0..self.image_size.x {
                pixels[pos..pos + 4].copy_from_slice(
                    self.color_list[pixel_iters[i as usize][j as usize]].as_slice(),
                );
                pos += 4;
            }
        }
        Handle::from_pixels(self.image_size.x as u32, self.image_size.y as u32, pixels)
    }

    fn build_iters_channel_line(&mut self) {
        *self.image_building.lock().unwrap() = true;
        let building = Arc::clone(&self.image_building);
        let pixel_iters = Arc::clone(&self.pixel_iters);
        let (size_x, size_y) = (self.image_size.x, self.image_size.y);
        let (min_x, max_x, min_y, max_y, n_workers) = (
            self.nparam.min_x,
            self.nparam.max_x,
            self.nparam.min_y,
            self.nparam.max_y,
            self.nparam.n_workers,
        );
        let pixel_x = (max_x - min_x) / (size_x as f64);
        let pixel_y = (max_y - min_y) / (size_y as f64);
        let elapsed = Arc::clone(&self.elapsed);
        let nparam = Arc::new(self.nparam.clone());
        thread::spawn(move || {
            let start = std::time::Instant::now();
            let (data_tx, data_rx) = channel();
            let mut stop_txs = Vec::new();
            let pool = ThreadPool::new(n_workers);
            let jj = Arc::new(Mutex::new(0));
            for n in 0..n_workers {
                let nparam_clone = Arc::clone(&nparam);
                let data_tx_clone = data_tx.clone();
                let (stop_tx, stop_rx) = channel();
                stop_txs.push(stop_tx);
                let jj_clone = Arc::clone(&jj);
                pool.execute(move || {
                    let frac: Box<dyn Iterator> = iterator(nparam_clone.as_ref());
                    let mut c = Complex(0.0, max_y - n as f64 * pixel_y);
                    while !stop_rx.try_recv().is_ok() {
                        let j;
                        {
                            let mut jj_lock = jj_clone.lock().unwrap();
                            if *jj_lock < size_y {
                                j = *jj_lock;
                                *jj_lock += 1;
                            } else {
                                break;
                            }
                        }
                        c.1 = max_y - j as f64 * pixel_y;
                        let mut line = vec![0_usize; size_x];
                        c.0 = min_x;
                        let mut i = 0;
                        while i < size_x {
                            c.0 += pixel_x;
                            line[i] = frac.iterate(&c);
                            i += 1;
                        }
                        data_tx_clone.send((j, line)).unwrap();
                    }
                });
            }
            drop(data_tx);
            for (j, line) in data_rx {
                pixel_iters.lock().unwrap()[j].copy_from_slice(&line);
                *elapsed.lock().unwrap() = format!("{:.2}", start.elapsed().as_secs_f32());
                if !*building.lock().unwrap() {
                    stop_txs.iter().for_each(|tx| {
                        let _ = tx.send(());
                    });
                }
            }
            *building.lock().unwrap() = false;
        });
    }

    #[allow(unused)]
    fn build_iters_channel_pixel(&mut self) {
        *self.image_building.lock().unwrap() = true;
        let building = Arc::clone(&self.image_building);
        let pixel_iters = Arc::clone(&self.pixel_iters);
        let (size_x, size_y) = (self.image_size.x, self.image_size.y);
        let (min_x, max_x, min_y, max_y, n_workers) = (
            self.nparam.min_x,
            self.nparam.max_x,
            self.nparam.min_y,
            self.nparam.max_y,
            self.nparam.n_workers,
        );
        let pixel_x = (max_x - min_x) / (size_x as f64);
        let pixel_y = (max_y - min_y) / (size_y as f64);
        let elapsed = Arc::clone(&self.elapsed);
        let nparam = Arc::new(self.nparam.clone());
        thread::spawn(move || {
            let start = std::time::Instant::now();
            let (data_tx, data_rx) = channel();
            let mut stop_txs = Vec::new();
            let pool = ThreadPool::new(n_workers);
            for n in 0..n_workers {
                let nparam_clone = Arc::clone(&nparam);
                let data_tx_clone = data_tx.clone();
                let (stop_tx, stop_rx) = channel();
                stop_txs.push(stop_tx);
                pool.execute(move || {
                    let frac: Box<dyn Iterator> = iterator(&*nparam_clone);
                    let mut j = n;
                    let mut c = Complex(0.0, max_y - j as f64 * pixel_y);
                    while j < size_y && !stop_rx.try_recv().is_ok() {
                        c.0 = min_x;
                        c.1 -= n_workers as f64 * pixel_y;
                        for i in 0..size_x {
                            c.0 += pixel_x;
                            data_tx_clone.send((i, j, frac.iterate(&c))).unwrap();
                        }
                        j += n_workers;
                    }
                });
            }
            drop(data_tx);
            let mut received = 0;
            for (i, j, c) in data_rx {
                pixel_iters.lock().unwrap()[j][i] = c;
                received += 1;
                if received == size_x {
                    *elapsed.lock().unwrap() = format!("{:.2}", start.elapsed().as_secs_f32());
                    received = 0;
                    if !*building.lock().unwrap() {
                        stop_txs.iter().for_each(|tx| {
                            let _ = tx.send(());
                        });
                    }
                }
            }
            *building.lock().unwrap() = false;
        });
    }

    #[allow(unused)]
    fn build_iters_arc(&mut self) {
        *self.image_building.lock().unwrap() = true;
        let building = Arc::clone(&self.image_building);
        let pixel_iters = Arc::clone(&self.pixel_iters);
        let (size_x, size_y) = (self.image_size.x, self.image_size.y);
        let (min_x, max_x, min_y, max_y, n_workers) = (
            self.nparam.min_x,
            self.nparam.max_x,
            self.nparam.min_y,
            self.nparam.max_y,
            self.nparam.n_workers,
        );
        let building_clone_0 = Arc::clone(&building);
        let elapsed = Arc::clone(&self.elapsed);
        thread::spawn(move || {
            let start = std::time::Instant::now();
            while *building_clone_0.lock().unwrap() {
                thread::sleep(Duration::from_millis(100));
                *elapsed.lock().unwrap() = format!("{:.1}", start.elapsed().as_secs_f32());
            }
        });

        let nparam = Arc::new(self.nparam.clone());
        thread::spawn(move || {
            let pool = ThreadPool::new(n_workers);
            for n in 0..n_workers {
                let nparam_clone = Arc::clone(&nparam);
                let building_clone = Arc::clone(&building);
                let iters_clone = Arc::clone(&pixel_iters);
                pool.execute(move || {
                    let frac: Box<dyn Iterator> = iterator(&*nparam_clone);
                    let pixel_x = (max_x - min_x) / (size_x as f64);
                    let pixel_y = (max_y - min_y) / (size_y as f64);
                    let coords_x: Vec<f64> =
                        (0..size_x).map(|i| min_x + (i as f64) * pixel_x).collect();
                    let mut c = Complex(0.0, 0.0);
                    let mut j = n;
                    while j < size_y && *building_clone.lock().unwrap() {
                        let mut line = vec![0_usize; size_x];
                        c.1 = max_y - j as f64 * pixel_y;
                        for i in 0..size_x {
                            c.0 = coords_x[i];
                            line[i] = frac.iterate(&c);
                        }
                        iters_clone.lock().unwrap()[j].copy_from_slice(&line);
                        j += n_workers;
                    }
                });
            }
            pool.join();
            *building.lock().unwrap() = false;
        });
    }

    #[allow(unused)]
    fn build_iters_arc0(&mut self) {
        *self.image_building.lock().unwrap() = true;
        let building = Arc::clone(&self.image_building);
        let pixel_iters = Arc::clone(&self.pixel_iters);
        let (size_x, size_y) = (self.image_size.x, self.image_size.y);
        let (min_x, max_x, min_y, max_y, n_workers) = (
            self.nparam.min_x,
            self.nparam.max_x,
            self.nparam.min_y,
            self.nparam.max_y,
            self.nparam.n_workers,
        );
        let building_clone_0 = Arc::clone(&building);
        let elapsed = Arc::clone(&self.elapsed);
        thread::spawn(move || {
            let start = std::time::Instant::now();
            while *building_clone_0.lock().unwrap() {
                thread::sleep(Duration::from_millis(100));
                *elapsed.lock().unwrap() = format!("{:.1}", start.elapsed().as_secs_f32());
            }
        });
        let nparam = Arc::new(self.nparam.clone());
        thread::spawn(move || {
            let mut handles = Vec::new();
            let jj = Arc::new(Mutex::new(0_usize));
            for _ in 0..n_workers {
                let nparam_clone = Arc::clone(&nparam);
                let building_clone = Arc::clone(&building);
                let iters_clone = Arc::clone(&pixel_iters);
                let jj_clone = Arc::clone(&jj);
                handles.push(thread::spawn(move || {
                    let frac: Box<dyn Iterator> = iterator(&*nparam_clone);
                    let pixel_x = (max_x - min_x) / (size_x as f64);
                    let pixel_y = (max_y - min_y) / (size_y as f64);
                    let coords_x: Vec<f64> =
                        (0..size_x).map(|i| min_x + (i as f64) * pixel_x).collect();
                    let mut c = Complex(0.0, 0.0);
                    loop {
                        let j;
                        {
                            let mut jj_lock = jj_clone.lock().unwrap();
                            if !*building_clone.lock().unwrap() || *jj_lock >= size_y {
                                break;
                            } else {
                                j = *jj_lock;
                                *jj_lock += 1;
                            }
                        };
                        let mut line = vec![0_usize; size_x];
                        c.1 = max_y - j as f64 * pixel_y;
                        for i in 0..size_x {
                            c.0 = coords_x[i];
                            // iters_clone.lock().unwrap()[j][i] = frac.iter_iter(&c);
                            line[i] = frac.iterate(&c);
                        }
                        iters_clone.lock().unwrap()[j].copy_from_slice(&line);
                    }
                }));
            }
            for handle in handles {
                handle.join().unwrap();
            }
            *building.lock().unwrap() = false;
        });
    }
}
