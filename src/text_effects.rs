pub fn bold(i: &str) -> String { format!("\x1B[1m{}\x1B[0m", i) }

pub fn dimmed(i: &str) -> String { format!("\x1B[2m{}\x1B[0m", i) }

pub fn italic(i: &str) -> String { format!("\x1B[3m{}\x1B[0m", i) }

pub fn underline(i: &str) -> String { format!("\x1B[4m{}\x1B[0m", i) }

pub fn blink(i: &str) -> String { format!("\x1B[5m{}\x1B[0m", i) }

pub fn reverse(i: &str) -> String { format!("\x1B[7m{}\x1B[0m", i) }

pub fn hidden(i: &str) -> String { format!("\x1B[8m{}\x1B[0m", i) }

pub fn stricken(i: &str) -> String { format!("\x1B[9m{}\x1B[0m", i) }
