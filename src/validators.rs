pub fn validate_is_digit(v: String) -> Result<(), String> {
  if v.parse::<u8>().is_ok() { return Ok(()); } 
  Err(String::from("The value did not contain valid digit from 0 to 255"))
}
