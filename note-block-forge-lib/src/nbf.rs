use crate::instruments::Instrument;

struct NoteBlockFile {
    metadata: Metadata,
    tracks: Vec<Track>,
    volume_envelopes: Vec<Envelope<u8>>,
    pan_envelopes: Vec<Envelope<i8>>,
}

struct Metadata {
    title: String,
    artist: String,
    transcriber: String,
    description: String,
}

enum Song {
    Static(StaticSong),
    Dynamic(DynamicSong),
}

/// A normal song that always plays the exact same from start to finish.
///
/// - A list of channels that are played in parallel.
/// - Each channel contains a list of [`TrackInstance`]s that are played in sequence.
struct StaticSong {
    channels: Vec<Channel>,
}

struct Channel {
    tracks: Vec<TrackInstance>,
}

/// A dynamic song that supports various ways to dynamically play different tracks.
///
/// It supports:
///
/// - Randomly switching and repeating sections of the song
/// - Dynamically enabling/disabling tracks to e.g. make the music more intense during battles
struct DynamicSong {
    sections: Vec<Section>,
}

struct Section {}

struct TrackInstance {
    instrument: Instrument,
    track: TrackIndex,
}

/// A collection of notes.
struct Track {
    start_offset: DeltaTicks,
    notes: Vec<Note>,
}

struct Envelope<T> {
    keyframes: Vec<Keyframe<T>>,
}

struct Keyframe<T> {
    /// How many ticks have passed since the last keyframe.
    delta: Ticks,
    /// The value at this keyframe.
    value: T,
}

/// A duration in number of Minecraft ticks (1 tick = 50ms).
///
/// Stored as a `u16` which limits the duration to 65535 ticks or ~54 minutes. Note that this limit
/// is not for the entire song, but only for a single track/envelope.
struct Ticks(u16);

struct DeltaTicks(i16);

struct TrackIndex(u16);

struct Note;
