use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub fn generate_compact_svg(
    lang_vec: &Vec<(String, u64)>,
    color_map: &HashMap<String, String>,
    output_path: &str,
) -> std::io::Result<()> {
    let svg_width = 880u32;
    let bar_height = 20u32;
    let padding_top = 24u32;
    let legend_line_height = 28u32;
    let legend_dot_radius = 6u32;
    let buffer = 16u32;

    let total_bytes: u64 = lang_vec.iter().map(|(_, b)| b).sum();

    let bar_y = padding_top;
    let legend_start_y = bar_y + bar_height + 16;

    let legend_columns = 4u32;
    let row_count = (lang_vec.len() as u32 + legend_columns - 1) / legend_columns;
    let svg_height = legend_start_y + row_count * legend_line_height + buffer;

    let mut file = File::create(output_path)?;
    let css = include_str!("./assets/style.css");

    writeln!(file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(
        file,
        r#"<svg width="{svg_width}" height="{svg_height}" xmlns="http://www.w3.org/2000/svg">"#
    )?;
    writeln!(file, r#"<style>{}</style>"#, css)?;

    // 背景: 透明（背景rectを削除）

    // ダーク/ライト両対応のテキストカラー用スタイル
    writeln!(
        file,
        r#"<style>
  text {{ fill: #24292f; }}
  @media (prefers-color-scheme: dark) {{
    text {{ fill: #e6edf3; }}
  }}
</style>"#
    )?;

    // stack-bar
    let current_x_start = 20u32;
    let bar_total_width = svg_width - 40;
    let bar_y_center = bar_y + bar_height / 2 - 5;

    writeln!(
        file,
        r#"
<defs>
    <clipPath id="roundedClip">
        <rect x="{current_x_start}" y="{bar_y_center}" width="{bar_total_width}" height="10" rx="5" ry="5" />
    </clipPath>
</defs>"#
    )?;

    // バーの背景（ダーク/ライト両対応）
    writeln!(
        file,
        r#"<rect width="{bar_total_width}" height="10" x="{current_x_start}" y="{bar_y_center}" rx="5" ry="5">
  <style>rect {{ fill: #e1e4e8; }} @media (prefers-color-scheme: dark) {{ rect {{ fill: #30363d; }} }}</style>
</rect>"#
    )?;

    writeln!(file, r#"<g clip-path="url(#roundedClip)">"#)?;
    let mut current_x = current_x_start;
    for (lang, bytes) in lang_vec {
        let percent = *bytes as f64 / total_bytes as f64;
        let bar_width = (percent * bar_total_width as f64).round() as u32;
        let color = color_map.get(lang).map(|s| s.as_str()).unwrap_or("#cccccc");
        writeln!(
            file,
            r#"    <rect x="{current_x}" y="{bar_y_center}" width="{bar_width}" height="10" fill="{color}"><title>{lang} {percent:.2}%</title></rect>"#,
            percent = percent * 100.0
        )?;
        current_x += bar_width;
    }
    writeln!(file, r#"</g>"#)?;

    // legend (4 columns)
    writeln!(file, r#"<g id="lang_legend">"#)?;
    let column_width = (svg_width - 40) / legend_columns;

    for (i, (lang, bytes)) in lang_vec.iter().enumerate() {
        let percent = *bytes as f64 / total_bytes as f64 * 100.0;
        let col = (i as u32) % legend_columns;
        let row = (i as u32) / legend_columns;
        let legend_x = 20 + col * column_width;
        let legend_y = legend_start_y + row * legend_line_height;
        let color = color_map.get(lang).map(|s| s.as_str()).unwrap_or("#cccccc");

        writeln!(
            file,
            r#"<circle cx="{cx}" cy="{cy}" r="{r}" fill="{color}" />
<text x="{tx}" y="{ty}" font-size="13" font-family="system-ui, -apple-system, sans-serif"><tspan font-weight="600">{lang}</tspan>  {percent:.2}%</text>"#,
            cx = legend_x + legend_dot_radius,
            cy = legend_y + 5,
            r = legend_dot_radius,
            tx = legend_x + legend_dot_radius * 2 + 8,
            ty = legend_y + 10,
        )?;
    }

    writeln!(file, r#"</g>"#)?;
    writeln!(file, "</svg>")?;

    Ok(())
}