use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub fn generate_compact_svg(
    lang_vec: &Vec<(String, u64)>,
    color_map: &HashMap<String, String>,
    output_path: &str,
) -> std::io::Result<()> {
    let svg_width = 410;
    let bar_height = 20;
    // タイトル削除に伴い、上部のパディングを縮小してサイズを調整
    let padding_top = 20;
    let legend_line_height = 24;
    let legend_dot_radius = 6;
    let buffer = 10;

    let total_bytes: u64 = lang_vec.iter().map(|(_, b)| b).sum();
    // タイトル分のオフセットを削除し、バーの位置を上に詰める
    let bar_y = padding_top;
    let legend_start_y = bar_y + bar_height + 20;

    let svg_height = legend_start_y + lang_vec.len() as u32 * legend_line_height / 2 + buffer;
    let mut file = File::create(output_path)?;
    let css = include_str!("./assets/style.css");

    writeln!(file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(
        file,
        r#"<svg width="{svg_width}" height="{svg_height}" xmlns="http://www.w3.org/2000/svg">"#
    )?;
    writeln!(file, r#"<style>{}</style>"#, css).expect("Failed to write CSS");

    // border & background (背景色を黒 '#000000' に変更)
    writeln!(
        file,
        r#"<rect id="border" x="1" y="1" width="{}" height="{}" fill='#000000' stroke-width="1" rx="15" ry="15" />"#,
        svg_width - 2,
        svg_height - 2
    )?;

    // stack-bar
    let mut current_x = 20;
    let bar_total_width = svg_width - 40;

    writeln!(
        file,
        r#"
<defs>
    <clipPath id="roundedClip">
        <rect id="bar_back" x="{current_x}" y="{bar_y}" width="{bar_total_width}" height="10" rx="5" ry="5">
        </rect>
    </clipPath>
</defs>"#
    )?;
    writeln!(
        file,
        r#"<rect width="{bar_total_width}" height="10" x="{current_x}" y="{bar_y}" rx="5" ry="5" fill='#ccc' />"#
    )?;
    writeln!(file, r#"<g clip-path="url(#roundedClip)">"#)?;

    for (lang, bytes) in lang_vec {
        let percent = *bytes as f64 / total_bytes as f64;
        let bar_width = (percent * bar_total_width as f64).round() as u32;
        let color = color_map.get(lang).map(|s| s.as_str()).unwrap_or("#cccccc");

        writeln!(
            file,
            r#"    <rect x="{current_x}" y="{bar_y}" width="{bar_width}" height="10" fill="{color}"> <title>{lang} {percent:.2}%</title> </rect>"#,
            percent = percent * 100.0
        )?;

        current_x += bar_width;
    }
    writeln!(file, r#"</g>"#)?;
    writeln!(file, r#"<g id="lang_legend">"#)?;

    // legend (2 columns)
    let legend_columns = 2;
    let column_width = svg_width / legend_columns - 10;
    for (i, (lang, bytes)) in lang_vec.iter().enumerate() {
        let percent = *bytes as f64 / total_bytes as f64 * 100.0;
        let col = i % legend_columns;
        let row = i / legend_columns;
        let legend_x = 20 + col * column_width;
        let legend_y = legend_start_y + (row as u32) * legend_line_height;

        let color = color_map.get(lang).map(|s| s.as_str()).unwrap_or("#cccccc");

        writeln!(
            file,
            r#"<circle cx="{x}" cy="{y}" r="{r}" fill="{color}" />
<text x="{}" y="{}" font-size="13" font-family="system-ui, -apple-system, sans-serif" fill='currentColor'><tspan>{lang}</tspan> {percent:.2}%</text>"#,
            legend_x + legend_dot_radius + 20,
            legend_y + 4,
            x = legend_x + 10,
            y = legend_y,
            r = legend_dot_radius,
            color = color,
            lang = lang,
            percent = percent
        )?;
    }
    writeln!(file, r#"</g>"#)?;
    writeln!(file, "</svg>")?;
    Ok(())
}
