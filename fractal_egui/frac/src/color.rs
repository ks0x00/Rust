use base::ColorType;

pub fn color_list(table_name: ColorType, iter: usize) -> Vec<[u8; 4]> {
    match table_name {
        ColorType::Rgb0 => rgb0(iter),
        ColorType::Rgb1 => rgb1(iter),
        ColorType::Rgb2 => rgb2(iter),
        ColorType::Rgb3 => rgb3(iter),
        ColorType::Rgb4 => rgb4(iter),
        ColorType::Gbr0 => gbr0(iter),
        ColorType::Gbr1 => gbr1(iter),
        ColorType::Gbr2 => gbr2(iter),
        ColorType::Gbr3 => gbr3(iter),
        ColorType::Gbr4 => gbr4(iter),
        ColorType::Brg0 => brg0(iter),
        ColorType::Brg1 => brg1(iter),
        ColorType::Brg2 => brg2(iter),
        ColorType::Brg3 => brg3(iter),
        ColorType::Brg4 => brg4(iter),
        ColorType::Rbg0 => rbg0(iter),
        ColorType::Rbg1 => rbg1(iter),
        ColorType::Rbg2 => rbg2(iter),
        ColorType::Rbg3 => rbg3(iter),
        ColorType::Rbg4 => rbg4(iter),
        ColorType::Bgr0 => bgr0(iter),
        ColorType::Bgr1 => bgr1(iter),
        ColorType::Bgr2 => bgr2(iter),
        ColorType::Bgr3 => bgr3(iter),
        ColorType::Bgr4 => bgr4(iter),
        ColorType::Grb0 => grb0(iter),
        ColorType::Grb1 => grb1(iter),
        ColorType::Grb2 => grb2(iter),
        ColorType::Grb3 => grb3(iter),
        ColorType::Grb4 => grb4(iter),
        ColorType::Hsb0 => hsb0(iter),
        ColorType::Hsb1 => hsb1(iter),
        ColorType::Hsb2 => hsb2(iter),
        ColorType::Hsb3 => hsb3(iter),
        ColorType::Hsb4 => hsb4(iter),
        ColorType::Hsl0 => hsl0(iter),
        ColorType::Hsl1 => hsl1(iter),
        ColorType::Hsl2 => hsl2(iter),
        ColorType::Hsl3 => hsl3(iter),
        ColorType::Hsl4 => hsl4(iter),
    }
}

fn rgb0(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = color as u32;
        table[i][0] = bgr as u8;
        table[i][1] = (bgr >> 8) as u8;
        table[i][2] = (bgr >> 16) as u8;
        color += color_unit;
    }
    table
}

fn rgb1(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = exchange4(color as u32);
        table[i][0] = bgr as u8;
        table[i][1] = (bgr >> 8) as u8;
        table[i][2] = (bgr >> 16) as u8;
        color += color_unit;
    }
    table
}

fn rgb2(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = rev_24bit(color as u32);
        table[i][0] = bgr as u8;
        table[i][1] = (bgr >> 8) as u8;
        table[i][2] = (bgr >> 16) as u8;
        color += color_unit;
    }
    table
}

fn rgb3(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = rev_exchange4(color as u32);
        table[i][0] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
        table[i][1] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
        table[i][2] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
        color += color_unit;
    }
    table
}

fn rgb4(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    for i in 0..iter + 1 {
        table[i][0] = i as u8;
        table[i][1] = (i >> 8) as u8;
        table[i][2] = (i >> 16) as u8;
    }
    table
}

fn gbr0(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = color as u32;
        table[i][0] = (bgr >> 8) as u8;
        table[i][1] = (bgr >> 16) as u8;
        table[i][2] = bgr as u8;
        color += color_unit;
    }
    table
}

fn gbr1(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = exchange4(color as u32);
        table[i][0] = (bgr >> 8) as u8;
        table[i][1] = (bgr >> 16) as u8;
        table[i][2] = bgr as u8;
        color += color_unit;
    }
    table
}

fn gbr2(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = rev_24bit(color as u32);
        table[i][0] = (bgr >> 8) as u8;
        table[i][1] = (bgr >> 16) as u8;
        table[i][2] = bgr as u8;
        color += color_unit;
    }
    table
}

fn gbr3(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = rev_exchange4(color as u32);
        table[i][0] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
        table[i][1] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
        table[i][2] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
        color += color_unit;
    }
    table
}

fn gbr4(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    for i in 0..iter + 1 {
        table[i][0] = (i >> 8) as u8;
        table[i][1] = (i >> 16) as u8;
        table[i][2] = i as u8;
    }
    table
}

fn brg0(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = color as u32;
        table[i][0] = (bgr >> 16) as u8;
        table[i][1] = bgr as u8;
        table[i][2] = (bgr >> 8) as u8;
        color += color_unit;
    }
    table
}

fn brg1(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = exchange4(color as u32);
        table[i][0] = (bgr >> 16) as u8;
        table[i][1] = bgr as u8;
        table[i][2] = (bgr >> 8) as u8;
        color += color_unit;
    }
    table
}

fn brg2(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = rev_24bit(color as u32);
        table[i][0] = (bgr >> 16) as u8;
        table[i][1] = bgr as u8;
        table[i][2] = (bgr >> 8) as u8;
        color += color_unit;
    }
    table
}

fn brg3(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = rev_exchange4(color as u32);
        table[i][0] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
        table[i][1] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
        table[i][2] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
        color += color_unit;
    }
    table
}

fn brg4(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    for i in 0..iter + 1 {
        table[i][0] = (i >> 16) as u8;
        table[i][1] = i as u8;
        table[i][2] = (i >> 8) as u8;
    }
    table
}

fn rbg0(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = color as u32;
        table[i][0] = (bgr & 0xFF) as u8;
        table[i][1] = (bgr >> 16 & 0xFF) as u8;
        table[i][2] = (bgr >> 8 & 0xFF) as u8;
        color += color_unit;
    }
    table
}

fn rbg1(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = exchange4(color as u32);
        table[i][0] = (bgr & 0xFF) as u8;
        table[i][1] = (bgr >> 16 & 0xFF) as u8;
        table[i][2] = (bgr >> 8 & 0xFF) as u8;
        color += color_unit;
    }
    table
}

fn rbg2(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = rev_24bit(color as u32);
        table[i][0] = (bgr & 0xFF) as u8;
        table[i][1] = (bgr >> 16 & 0xFF) as u8;
        table[i][2] = (bgr >> 8 & 0xFF) as u8;
        color += color_unit;
    }
    table
}

fn rbg3(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = rev_exchange4(color as u32);
        table[i][0] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
        table[i][1] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
        table[i][2] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
        color += color_unit;
    }
    table
}

fn rbg4(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    for i in 0..iter + 1 {
        table[i][0] = i as u8;
        table[i][1] = (i >> 16) as u8;
        table[i][2] = (i >> 8) as u8;
    }
    table
}

fn bgr0(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = color as u32;
        table[i][0] = (bgr >> 16 & 0xFF) as u8;
        table[i][1] = (bgr >> 8 & 0xFF) as u8;
        table[i][2] = (bgr & 0xFF) as u8;
        color += color_unit;
    }
    table
}

fn bgr1(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = exchange4(color as u32);
        table[i][0] = (bgr >> 16 & 0xFF) as u8;
        table[i][1] = (bgr >> 8 & 0xFF) as u8;
        table[i][2] = (bgr & 0xFF) as u8;
        color += color_unit;
    }
    table
}

fn bgr2(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = rev_24bit(color as u32);
        table[i][0] = (bgr >> 16 & 0xFF) as u8;
        table[i][1] = (bgr >> 8 & 0xFF) as u8;
        table[i][2] = (bgr & 0xFF) as u8;
        color += color_unit;
    }
    table
}

fn bgr3(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = rev_exchange4(color as u32);
        table[i][0] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
        table[i][1] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
        table[i][2] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
        color += color_unit;
    }
    table
}

fn bgr4(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    for i in 0..iter + 1 {
        table[i][0] = (i >> 16) as u8;
        table[i][1] = (i >> 8) as u8;
        table[i][2] = i as u8;
    }
    table
}

fn grb0(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = color as u32;
        table[i][0] = (bgr >> 8 & 0xFF) as u8;
        table[i][1] = (bgr & 0xFF) as u8;
        table[i][2] = (bgr >> 16 & 0xFF) as u8;
        color += color_unit;
    }
    table
}

fn grb1(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = exchange4(color as u32);
        table[i][0] = (bgr >> 8 & 0xFF) as u8;
        table[i][1] = (bgr & 0xFF) as u8;
        table[i][2] = (bgr >> 16 & 0xFF) as u8;
        color += color_unit;
    }
    table
}

fn grb2(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = rev_24bit(color as u32);
        table[i][0] = (bgr >> 8 & 0xFF) as u8;
        table[i][1] = (bgr & 0xFF) as u8;
        table[i][2] = (bgr >> 16 & 0xFF) as u8;
        color += color_unit;
    }
    table
}

fn grb3(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let bgr = rev_exchange4(color as u32);
        table[i][0] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
        table[i][1] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
        table[i][2] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
        color += color_unit;
    }
    table
}

fn grb4(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    for i in 0..iter + 1 {
        table[i][0] = (i >> 8) as u8;
        table[i][1] = i as u8;
        table[i][2] = (i >> 16) as u8;
    }
    table
}

fn hsb0(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = 0xFFFFFF as f64 / iter as f64;
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let c = color as u32;
        (table[i][0], table[i][1], table[i][2]) =
            hsb2rgb(pol2((c >> 8) as u8), pol2(c as u8), pol2((c >> 16) as u8));
        color += color_unit;
    }
    table
}

fn hsb1(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = 0xFFFFFF as f64 / iter as f64;
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let c = exchange4(color as u32);
        (table[i][0], table[i][1], table[i][2]) =
            hsb2rgb(pol2((c >> 8) as u8), pol2(c as u8), pol2((c >> 16) as u8));
        color += color_unit;
    }
    table
}
fn hsb2(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = 0xFFFFFF as f64 / iter as f64;
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let c = rev_24bit(color as u32);
        (table[i][0], table[i][1], table[i][2]) =
            hsb2rgb(pol2((c >> 8) as u8), pol2(c as u8), pol2((c >> 16) as u8));
        color += color_unit;
    }
    table
}
fn hsb3(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = 0xFFFFFF as f64 / iter as f64;
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let c = rev_exchange4(color as u32);
        (table[i][0], table[i][1], table[i][2]) =
            hsb2rgb(pol2((c >> 8) as u8), pol2(c as u8), pol2((c >> 16) as u8));
        color += color_unit;
    }
    table
}
fn hsb4(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    for i in 0..iter + 1 {
        (table[i][0], table[i][1], table[i][2]) =
            hsb2rgb(pol2((i >> 8) as u8), pol2(i as u8), pol2((i >> 16) as u8));
    }
    table
}

fn hsl0(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = 0xFFFFFF as f64 / iter as f64;
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let c = color as u32;
        (table[i][0], table[i][1], table[i][2]) =
            hsl2rgb(pol1((c >> 8) as u8), pol1((c >> 16) as u8), pol1(c as u8));
        color += color_unit;
    }
    table
}

fn hsl1(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = 0xFFFFFF as f64 / iter as f64;
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let c = exchange4(color as u32);
        (table[i][0], table[i][1], table[i][2]) =
            hsl2rgb(pol1((c >> 8) as u8), pol1((c >> 16) as u8), pol1(c as u8));
        color += color_unit;
    }
    table
}

fn hsl2(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = 0xFFFFFF as f64 / iter as f64;
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let c = rev_24bit(color as u32);
        (table[i][0], table[i][1], table[i][2]) =
            hsl2rgb(pol1((c >> 8) as u8), pol1((c >> 16) as u8), pol1(c as u8));
        color += color_unit;
    }
    table
}

fn hsl3(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = 0xFFFFFF as f64 / iter as f64;
    let mut color = 0.0;
    for i in 0..iter + 1 {
        let c = rev_exchange4(color as u32);
        (table[i][0], table[i][1], table[i][2]) =
            hsl2rgb(pol2((c >> 8) as u8), pol2(c as u8), pol2((c >> 16) as u8));
        color += color_unit;
    }
    table
}

fn hsl4(iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    for i in 0..iter + 1 {
        (table[i][0], table[i][1], table[i][2]) =
            hsl2rgb(pol1((i >> 8) as u8), pol1((i >> 16) as u8), pol1(i as u8));
    }
    table
}

fn hsb2rgb(mut h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    // let h = (hsv >> 16) as u8 as f64 * 360.0 / 255.0;
    // let s = (hsv >> 8) as u8 as f64 / 255.0;
    // let v = hsv as u8 as f64 / 255.0;
    h *= 360.0;
    let c = s * v;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (rp, gp, bp) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (
        ((rp + m) * 255.0) as u8,
        ((gp + m) * 255.0) as u8,
        ((bp + m) * 255.0) as u8,
    )
}

fn hsl2rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    if s == 0.0 {
        let r = (255.0 * l) as u8; // achromatic
        (r, r, r)
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        (
            (255.0 * hue2rgb(p, q, h + 1.0 / 3.0)) as u8,
            (255.0 * hue2rgb(p, q, h)) as u8,
            (255.0 * hue2rgb(p, q, h - 1.0 / 3.0)) as u8,
        )
    }
}

fn hue2rgb(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 {
        t += 1.0;
    } else if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (4.0 - 6.0 * t)
    } else {
        p
    }
}

fn pol1(x: u8) -> f64 {
    1.0 - x as f64 / 255.0
}

fn pol2(x: u8) -> f64 {
    (1.0 - x as f64 / 255.0).powi(2)
}

fn exchange4(x: u32) -> u32 {
    ((x & 0xF0F0F) << 4) | ((x >> 4) & 0xF0F0F)
}

fn rev_exchange4(mut x: u32) -> u32 {
    x = ((x & 0xFFFF) << 16) | ((x >> 16) & 0xFFFF);
    x = ((x & 0xFF00FF) << 8) | ((x >> 8) & 0xFF00FF);
    x = ((x & 0xF0F0F0F) << 4) | ((x >> 4) & 0xF0F0F0F);
    (((x & 0x33333333) << 2) | ((x >> 2) & 0x33333333)) >> 8
}

/*
fn rev_8bit(mut x: u32) -> u32 {
    // 0123 4567
    x = ((x & 0xF) << 4) | ((x >> 20) & 0xF); // 4567 0123
    x = ((x & 0x33) << 2) | ((x >> 2) & 0x33); // 6745 2301
    ((x & 0x55) << 1) | ((x >> 1) & 0x55) // 7654 3210
}
*/

fn rev_24bit(mut x: u32) -> u32 {
    x = ((x & 0xFFFF) << 16) | ((x >> 16) & 0xFFFF);
    x = ((x & 0xFF00FF) << 8) | ((x >> 8) & 0xFF00FF);
    x = ((x & 0xF0F0F0F) << 4) | ((x >> 4) & 0xF0F0F0F);
    x = ((x & 0x33333333) << 2) | ((x >> 2) & 0x33333333);
    (((x & 0x55555555) << 1) | ((x >> 1) & 0x55555555)) >> 8
}
