# Edit Clipboard

A small helper program that allows editing of clipboard contents with helix.

A string of "flags" can be used to invoke preprocessor(s). This can be found in `src/preprocessor/transform.rs`. Some options can be changed at the top of the `main.rs` file.

## Requires

- [Helix](https://helix-editor.com/)
- [Alacritty](https://alacritty.org/)

## Todo

- Implement Processor Ordering.
  - As of now text is sent through the processor in flag order.
- Use clap for arguments
- Document preprocessor options in cli
