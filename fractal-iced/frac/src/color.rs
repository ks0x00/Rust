use base::{ColorType, ColorVar};
use param::NParameter;

pub fn color_list(nparam: &NParameter) -> Vec<[u8; 4]> {
    match nparam.color_type {
        ColorType::Rgb => rgb(nparam.color_var, nparam.max_iter),
        ColorType::Gbr => gbr(nparam.color_var, nparam.max_iter),
        ColorType::Brg => brg(nparam.color_var, nparam.max_iter),
        ColorType::Rbg => rbg(nparam.color_var, nparam.max_iter),
        ColorType::Bgr => bgr(nparam.color_var, nparam.max_iter),
        ColorType::Grb => grb(nparam.color_var, nparam.max_iter),
        ColorType::Cmy => cmy(nparam.color_var, nparam.max_iter),
        ColorType::Myc => myc(nparam.color_var, nparam.max_iter),
        ColorType::Ycm => ycm(nparam.color_var, nparam.max_iter),
        ColorType::Cym => cym(nparam.color_var, nparam.max_iter),
        ColorType::Ymc => ymc(nparam.color_var, nparam.max_iter),
        ColorType::Mcy => mcy(nparam.color_var, nparam.max_iter),
        ColorType::Hsb => hsb(nparam.color_var, nparam.max_iter),
        ColorType::Hsl => hsl(nparam.color_var, nparam.max_iter),
    }
}

fn rgb(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let bgr = color as u32;
                table[i][0] = bgr as u8;
                table[i][1] = (bgr >> 8) as u8;
                table[i][2] = (bgr >> 16) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let bgr = exchange4(color as u32);
                table[i][0] = bgr as u8;
                table[i][1] = (bgr >> 8) as u8;
                table[i][2] = (bgr >> 16) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let bgr = rev_24bit(color as u32);
                table[i][0] = bgr as u8;
                table[i][1] = (bgr >> 8) as u8;
                table[i][2] = (bgr >> 16) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let bgr = rev_exchange4(color as u32);
                table[i][0] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][1] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][2] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                table[i][0] = i as u8;
                table[i][1] = (i >> 8) as u8;
                table[i][2] = (i >> 16) as u8;
            }
        }
    }
    table
}

fn gbr(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let bgr = color as u32;
                table[i][0] = (bgr >> 8) as u8;
                table[i][1] = (bgr >> 16) as u8;
                table[i][2] = bgr as u8;
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let bgr = exchange4(color as u32);
                table[i][0] = (bgr >> 8) as u8;
                table[i][1] = (bgr >> 16) as u8;
                table[i][2] = bgr as u8;
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let bgr = rev_24bit(color as u32);
                table[i][0] = (bgr >> 8) as u8;
                table[i][1] = (bgr >> 16) as u8;
                table[i][2] = bgr as u8;
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let bgr = rev_exchange4(color as u32);
                table[i][0] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][1] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][2] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                table[i][0] = (i >> 8) as u8;
                table[i][1] = (i >> 16) as u8;
                table[i][2] = i as u8;
            }
        }
    }
    table
}

fn brg(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let bgr = color as u32;
                table[i][0] = (bgr >> 16) as u8;
                table[i][1] = bgr as u8;
                table[i][2] = (bgr >> 8) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let bgr = exchange4(color as u32);
                table[i][0] = (bgr >> 16) as u8;
                table[i][1] = bgr as u8;
                table[i][2] = (bgr >> 8) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let bgr = rev_24bit(color as u32);
                table[i][0] = (bgr >> 16) as u8;
                table[i][1] = bgr as u8;
                table[i][2] = (bgr >> 8) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let bgr = rev_exchange4(color as u32);
                table[i][0] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][1] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][2] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                table[i][0] = (i >> 16) as u8;
                table[i][1] = i as u8;
                table[i][2] = (i >> 8) as u8;
            }
        }
    }
    table
}

fn rbg(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let bgr = color as u32;
                table[i][0] = (bgr & 0xFF) as u8;
                table[i][1] = (bgr >> 16 & 0xFF) as u8;
                table[i][2] = (bgr >> 8 & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let bgr = exchange4(color as u32);
                table[i][0] = (bgr & 0xFF) as u8;
                table[i][1] = (bgr >> 16 & 0xFF) as u8;
                table[i][2] = (bgr >> 8 & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let bgr = rev_24bit(color as u32);
                table[i][0] = (bgr & 0xFF) as u8;
                table[i][1] = (bgr >> 16 & 0xFF) as u8;
                table[i][2] = (bgr >> 8 & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let bgr = rev_exchange4(color as u32);
                table[i][0] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][1] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][2] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                table[i][0] = i as u8;
                table[i][1] = (i >> 16) as u8;
                table[i][2] = (i >> 8) as u8;
            }
        }
    }
    table
}

fn bgr(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let bgr = color as u32;
                table[i][0] = (bgr >> 16 & 0xFF) as u8;
                table[i][1] = (bgr >> 8 & 0xFF) as u8;
                table[i][2] = (bgr & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let bgr = exchange4(color as u32);
                table[i][0] = (bgr >> 16 & 0xFF) as u8;
                table[i][1] = (bgr >> 8 & 0xFF) as u8;
                table[i][2] = (bgr & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let bgr = rev_24bit(color as u32);
                table[i][0] = (bgr >> 16 & 0xFF) as u8;
                table[i][1] = (bgr >> 8 & 0xFF) as u8;
                table[i][2] = (bgr & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let bgr = rev_exchange4(color as u32);
                table[i][0] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][1] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][2] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                table[i][0] = (i >> 16) as u8;
                table[i][1] = (i >> 8) as u8;
                table[i][2] = i as u8;
            }
        }
    }
    table
}

fn grb(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let bgr = color as u32;
                table[i][0] = (bgr >> 8 & 0xFF) as u8;
                table[i][1] = (bgr & 0xFF) as u8;
                table[i][2] = (bgr >> 16 & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let bgr = exchange4(color as u32);
                table[i][0] = (bgr >> 8 & 0xFF) as u8;
                table[i][1] = (bgr & 0xFF) as u8;
                table[i][2] = (bgr >> 16 & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let bgr = rev_24bit(color as u32);
                table[i][0] = (bgr >> 8 & 0xFF) as u8;
                table[i][1] = (bgr & 0xFF) as u8;
                table[i][2] = (bgr >> 16 & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let bgr = rev_exchange4(color as u32);
                table[i][0] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][1] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][2] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                table[i][0] = (i >> 8) as u8;
                table[i][1] = i as u8;
                table[i][2] = (i >> 16) as u8;
            }
        }
    }
    table
}

fn hsb(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = 0xFFFFFF as f64 / iter as f64;
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let c = color as u32;
                (table[i][0], table[i][1], table[i][2]) =
                    hsb2rgb(pol2((c >> 8) as u8), pol2(c as u8), pol2((c >> 16) as u8));
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let c = exchange4(color as u32);
                (table[i][0], table[i][1], table[i][2]) =
                    hsb2rgb(pol2((c >> 8) as u8), pol2(c as u8), pol2((c >> 16) as u8));
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let c = rev_24bit(color as u32);
                (table[i][0], table[i][1], table[i][2]) =
                    hsb2rgb(pol2((c >> 8) as u8), pol2(c as u8), pol2((c >> 16) as u8));
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let c = rev_exchange4(color as u32);
                (table[i][0], table[i][1], table[i][2]) =
                    hsb2rgb(pol2((c >> 8) as u8), pol2(c as u8), pol2((c >> 16) as u8));
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                (table[i][0], table[i][1], table[i][2]) =
                    hsb2rgb(pol2((i >> 8) as u8), pol2(i as u8), pol2((i >> 16) as u8));
            }
        }
    }
    table
}

fn hsl(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = 0xFFFFFF as f64 / iter as f64;
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let c = color as u32;
                (table[i][0], table[i][1], table[i][2]) =
                    hsl2rgb(pol1((c >> 8) as u8), pol1((c >> 16) as u8), pol1(c as u8));
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let c = exchange4(color as u32);
                (table[i][0], table[i][1], table[i][2]) =
                    hsl2rgb(pol1((c >> 8) as u8), pol1((c >> 16) as u8), pol1(c as u8));
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let c = rev_24bit(color as u32);
                (table[i][0], table[i][1], table[i][2]) =
                    hsl2rgb(pol1((c >> 8) as u8), pol1((c >> 16) as u8), pol1(c as u8));
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let c = rev_exchange4(color as u32);
                (table[i][0], table[i][1], table[i][2]) =
                    hsl2rgb(pol2((c >> 8) as u8), pol2(c as u8), pol2((c >> 16) as u8));
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                (table[i][0], table[i][1], table[i][2]) =
                    hsl2rgb(pol1((i >> 8) as u8), pol1((i >> 16) as u8), pol1(i as u8));
            }
        }
    }
    table
}

fn cmy(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let bgr = !(color as u32);
                table[i][0] = bgr as u8;
                table[i][1] = (bgr >> 8) as u8;
                table[i][2] = (bgr >> 16) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let bgr = !exchange4(color as u32);
                table[i][0] = bgr as u8;
                table[i][1] = (bgr >> 8) as u8;
                table[i][2] = (bgr >> 16) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let bgr = !rev_24bit(color as u32);
                table[i][0] = bgr as u8;
                table[i][1] = (bgr >> 8) as u8;
                table[i][2] = (bgr >> 16) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let bgr = !rev_exchange4(color as u32);
                table[i][0] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][1] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][2] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                table[i][0] = !(i as u8);
                table[i][1] = !((i >> 8) as u8);
                table[i][2] = !((i >> 16) as u8);
            }
        }
    }
    table
}

fn myc(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let bgr = !(color as u32);
                table[i][0] = (bgr >> 8) as u8;
                table[i][1] = (bgr >> 16) as u8;
                table[i][2] = bgr as u8;
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let bgr = !exchange4(color as u32);
                table[i][0] = (bgr >> 8) as u8;
                table[i][1] = (bgr >> 16) as u8;
                table[i][2] = bgr as u8;
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let bgr = !rev_24bit(color as u32);
                table[i][0] = (bgr >> 8) as u8;
                table[i][1] = (bgr >> 16) as u8;
                table[i][2] = bgr as u8;
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let bgr = !rev_exchange4(color as u32);
                table[i][0] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][1] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][2] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                table[i][0] = !((i >> 8) as u8);
                table[i][1] = !((i >> 16) as u8);
                table[i][2] = !(i as u8);
            }
        }
    }
    table
}

fn ycm(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let bgr = !(color as u32);
                table[i][0] = (bgr >> 16) as u8;
                table[i][1] = bgr as u8;
                table[i][2] = (bgr >> 8) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let bgr = !exchange4(color as u32);
                table[i][0] = (bgr >> 16) as u8;
                table[i][1] = bgr as u8;
                table[i][2] = (bgr >> 8) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let bgr = !rev_24bit(color as u32);
                table[i][0] = (bgr >> 16) as u8;
                table[i][1] = bgr as u8;
                table[i][2] = (bgr >> 8) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let bgr = !rev_exchange4(color as u32);
                table[i][0] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][1] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][2] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                table[i][0] = !((i >> 16) as u8);
                table[i][1] = !(i as u8);
                table[i][2] = !((i >> 8) as u8);
            }
        }
    }
    table
}

fn cym(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let bgr = !(color as u32);
                table[i][0] = (bgr & 0xFF) as u8;
                table[i][1] = (bgr >> 16 & 0xFF) as u8;
                table[i][2] = (bgr >> 8 & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let bgr = !exchange4(color as u32);
                table[i][0] = (bgr & 0xFF) as u8;
                table[i][1] = (bgr >> 16 & 0xFF) as u8;
                table[i][2] = (bgr >> 8 & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let bgr = !rev_24bit(color as u32);
                table[i][0] = (bgr & 0xFF) as u8;
                table[i][1] = (bgr >> 16 & 0xFF) as u8;
                table[i][2] = (bgr >> 8 & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let bgr = !rev_exchange4(color as u32);
                table[i][0] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][1] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][2] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                table[i][0] = !(i as u8);
                table[i][1] = !((i >> 16) as u8);
                table[i][2] = !((i >> 8) as u8);
            }
        }
    }
    table
}

fn ymc(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let bgr = !(color as u32);
                table[i][0] = (bgr >> 16) as u8;
                table[i][1] = (bgr >> 8) as u8;
                table[i][2] = bgr as u8;
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let bgr = !exchange4(color as u32);
                table[i][0] = (bgr >> 16) as u8;
                table[i][1] = (bgr >> 8) as u8;
                table[i][2] = bgr as u8;
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let bgr = !rev_24bit(color as u32);
                table[i][0] = (bgr >> 16) as u8;
                table[i][1] = (bgr >> 8) as u8;
                table[i][2] = bgr as u8;
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let bgr = !rev_exchange4(color as u32);
                table[i][0] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][1] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][2] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                table[i][0] = !((i >> 16) as u8);
                table[i][1] = !((i >> 8) as u8);
                table[i][2] = !(i as u8);
            }
        }
    }
    table
}

fn mcy(color_var: ColorVar, iter: usize) -> Vec<[u8; 4]> {
    let mut table = vec![[0, 0, 0, 255]; iter + 1];
    let color_unit = (0xFFFFFF as f64) / (iter as f64);
    let mut color = 0.0;
    match color_var {
        ColorVar::Var0 => {
            for i in 0..iter + 1 {
                let bgr = !(color as u32);
                table[i][0] = (bgr >> 8 & 0xFF) as u8;
                table[i][1] = (bgr & 0xFF) as u8;
                table[i][2] = (bgr >> 16 & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var1 => {
            for i in 0..iter + 1 {
                let bgr = !exchange4(color as u32);
                table[i][0] = (bgr >> 8 & 0xFF) as u8;
                table[i][1] = (bgr & 0xFF) as u8;
                table[i][2] = (bgr >> 16 & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var2 => {
            for i in 0..iter + 1 {
                let bgr = !rev_24bit(color as u32);
                table[i][0] = (bgr >> 8 & 0xFF) as u8;
                table[i][1] = (bgr & 0xFF) as u8;
                table[i][2] = (bgr >> 16 & 0xFF) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var3 => {
            for i in 0..iter + 1 {
                let bgr = !rev_exchange4(color as u32);
                table[i][0] = (255.0 - 255.0 * ((bgr >> 8) as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][1] = (255.0 - 255.0 * (bgr as u8 as f64 / 255.0).sqrt()) as u8;
                table[i][2] = (255.0 - 255.0 * ((bgr >> 16) as u8 as f64 / 255.0).sqrt()) as u8;
                color += color_unit;
            }
        }
        ColorVar::Var4 => {
            for i in 0..iter + 1 {
                table[i][0] = !((i >> 8) as u8);
                table[i][1] = !(i as u8);
                table[i][2] = !((i >> 16) as u8);
            }
        }
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
