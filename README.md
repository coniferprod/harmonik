# harmonik

Generate harmonic levels for the Kawai K5000 Advanced Additive Synthesizer.

This program uses a parameterized method to generate amplitude levels for
additive harmonics. The method was patented as [U.S. patent 6143974A](https://patents.google.com/patent/US6143974A/en),
but the patent has expired in 2019.

## Usage

* [Install Rust](https://www.rust-lang.org/learn/get-started).
* Clone this repository: `git clone https://github.com/coniferprod/harmonik`
* Change to the program directory: `cd harmonik`
* Execute the program: `cargo run`

The program creates commands to send MIDI System Exclusive messages that set the harmonic levels
(first group only), using the [SendMIDI](https://github.com/gbevin/SendMIDI) program.

## Examples

Specify either one of the standard analog waveforms, or a custom waveform with six parameters:

    cargo run -- --waveform sine

produces the levels for a sine waveform, while

    cargo run -- --waveform custom 3.0,1.0,0.0,0.48,2.0,0.0,0.035

produces the levels for an "analog square" waveform.

To create a set of waveforms with completely random levels:

    cargo run -- --waveform random

You can specify the device and MIDI channel that will be printed into the SendMIDI command,
using the --channel and -device parameters:

    cargo run -- --channel 1 --device "Studio 1824"

If you don't specify these, they default to channel 1 and "MIDI Out".
