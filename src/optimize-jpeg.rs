fn optimize_image(img: &bytes::Bytes) -> Result<bytes::Bytes> {
  let mut child = Command::new("jpegoptim")
    .args(&["-q", "--strip-none", "--stdout", "--stdin"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;
  child
    .stdin
    .as_mut()
    .ok_or_else(|| anyhow!("no stdin"))?
    .write_all(img)?;
  let output = child.wait_with_output()?;
  if output.status.success() {
    Ok(bytes::Bytes::from(output.stdout))
  } else {
    Err(anyhow!("jpegoptim failed"))
  }
}
