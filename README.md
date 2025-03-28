# NoteBlockForge

A DAW for Minecraft Note Block Music.

Also comes with a handy CLI tool.

## Crates

This project is split up into a few crates:

### [note-block-forge](note-block-forge/README.md)

The graphical editor, with all the other utility tools built-in as well.

### [note-block-forge-cli](note-block-forge-cli/README.md)

Some utility CLI tools for e.g. just listening to a song or exporting to a datapack.

### [note-block-forge-lib](note-block-forge-lib/README.md)

A library that is used by the two above.

## Notes

- [ ] Assets will have to be committed to the repository if I want it to build on the CI

## File Format

The file format for any NoteBlockForge file is `.nbf` and can contain various things:

- [ ] Version for compatibility
- [ ] Metadata
  - [ ] Name
  - [ ] Artist
  - [ ] Transcriber

- [ ] Tracks
  - [ ] Name
  - [ ] TickDivision
  - [ ]
  - [ ] Volume
    - [ ] default 100%
    - [ ] Volume Envelope (optional)
      - [ ] Envelope Scale
      - [ ] Envelope Offset
  - [ ] Pan
    - [ ] default 0%
    - [ ] Pan Envelope (optional)
      - [ ] Envelope Scale
- [ ] Envelopes
  - [ ] List of ticks + values
