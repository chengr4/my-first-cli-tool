pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
  for line in content.lines() {
      if line.contains(pattern) {
          // writer: target to write to
          writeln!(writer, "{}", line).unwrap();
      }
  }
}