use std::io::{self, Read, Write, stdin, stdout};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Debug, Clone)]
pub struct TerminalFontProfile {
	pub font_family: String,
	pub cell_width_px: u32,
	pub cell_height_px: u32
}

pub fn query_font_profile() -> io::Result<TerminalFontProfile> {
	enable_raw_mode()?;
	let mut stdout = stdout();
	let mut stdin = stdin();
	
	stdout.write_all(b"\x1b[16t\x1b]50;?\x07")?;
	stdout.flush()?;
	
	let mut response = Vec::new();
	let mut buf = [0u8; 128];
	
	stdout.write_all(b"\x1b[x")?;
	stdout.flush()?;
	
	let mut bytes_read = 0;
	
	while bytes_read < 256  {
		let n = stdin.read(&mut buf)?;
		
		if n == 0 { break; }
		
		response.extend_from_slice(&buf[..n]);
		bytes_read += n;
		
		if response.ends_with(b"c") || response.contains(&b't') {
			break;
		}
	}
	
	disable_raw_mode()?;
	
	let resp_str = String::from_utf8_lossy(&response);
	let (cell_width_px, cell_height_px) = parse_cell_size(&resp_str).unwrap_or((10, 20));
	let font_family = parse_font_name(&resp_str).unwrap_or_else(|| "monospace".to_string());
	
	Ok(TerminalFontProfile { 
		font_family, 
		cell_width_px, 
		cell_height_px 
	})
}

fn parse_cell_size(input: &str) -> Option<(u32, u32)> {
	//"\x1b[6;<h>;<w>t"
	let start = input.find("\x1b[6;")?;
	let rest = &input[start + 4..];
	let end = rest.find('t')?;
	let parts: Vec<&str> = rest[..end].split(';').collect();
	
	if parts.len() == 2 {
		let h = parts[0].parse().ok()?;
		let w = parts[1].parse().ok()?;
		
		return Some((w, h))
	}
	
	None
}

fn parse_font_name(input: &str) -> Option<String> {
	//"\x1b]50;<name>\x07" or "\x1b]50;<name>\x1b\\"
	let start = input.find("\x1b]50;")?;
	let rest = &input[start + 5..];
	let end = rest.find('\x07').or_else(|| rest.find("\x1b\\"))?;
	
	Some(rest[..end].to_string())
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_parse_cell_size_valid_csi_16t() {
		let response = "\x1b[6;24;12t";
		let result = parse_cell_size(response);
		
		assert_eq!(result, Some((12, 24)), "Width should be 12px, Height should be 24px");
	}
	
	#[test]
	fn test_parse_cell_size_noisy_stream() {
		// Simulates query sequence intermingled with secondary queries (e.g., ESC [ c)
		let response = "\x1b[?1;2c\x1b[6;20;10t\x1b]50;DejaVu Sans Mono\x07";
		let result = parse_cell_size(response);
		
		assert_eq!(result, Some((10, 20)));
	}
	
	#[test]
	fn test_parse_cell_size_malformed() {
		assert_eq!(parse_cell_size("\x1b[6;20t"), None, "Missing second dimension parameter");
		assert_eq!(parse_cell_size("\x1b[6;abc;10t"), None, "Non-numeric height component");
		assert_eq!(parse_cell_size(""), None, "Empty buffer input");
	}
	
	#[test]
	fn test_parse_font_name_osc_50_bel_terminated() {
		// OSC 50 string terminated with BEL (\x07)
		let response = "\x1b]50;DejaVu Sans Mono\x07";
		let result = parse_font_name(response);
		
		assert_eq!(result.as_deref(), Some("DejaVu Sans Mono"));
	}
	
	#[test]
	fn test_parse_font_name_osc_50_st_terminated() {
		// OSC 50 string terminated with String Terminator (ST = \x1b\)
		let response = "\x1b]50;DejaVu Sans Mono\x1b\\";
		let result = parse_font_name(response);
		
		assert_eq!(result.as_deref(), Some("DejaVu Sans Mono"));
	}
	
	#[test]
	fn test_parse_font_name_missing_or_corrupted() {
		assert_eq!(parse_font_name("\x1b]50;UnterminatedFont"), None);
		assert_eq!(parse_font_name("random text string"), None);
	}
	
	#[test]
	fn test_profile_font_size_scaling() {
		let profile = TerminalFontProfile {
			font_family: "Hack".to_string(),
			cell_width_px: 10,
			cell_height_px: 20,
		};
		
		// H1 header multiplier (3.0x cell height)
		let h1_size = profile.cell_height_px as f32 * 3.0;
		assert_eq!(h1_size, 60.0);
		
		// H2 header multiplier (2.0x cell height)
		let h2_size = profile.cell_height_px as f32 * 2.0;
		assert_eq!(h2_size, 40.0);
	}
	
	#[test]
	fn test_fallback_defaults() {
		// Ensures empty or timed-out parsing defaults to reasonable fallback dimensions
		let raw_response = "unsupported sequence response";
		let (width, height) = parse_cell_size(raw_response).unwrap_or((10, 20));
		let font_family = parse_font_name(raw_response).unwrap_or_else(|| "monospace".to_string());
		
		assert_eq!(width, 10);
		assert_eq!(height, 20);
		assert_eq!(font_family, "monospace");
	}
}