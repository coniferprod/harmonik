# harmonik

Generate harmonic levels for the Kawai K5000 Advanced Additive Synthesizer.

This program uses a parameterized method to generate amplitude levels for
additive harmonics. The method was patented as [U.S. patent 6143974A](https://patents.google.com/patent/US6143974A/en),
but the patent has expired in 2019.

## Examples

Specify either one of the standard analog waveforms, or a custom waveform with six parameters:

    cargo run -- --waveform sine

produces the levels for a sine waveform, while

    cargo run -- --waveform custom 3.0,1.0,0.0,0.48,2.0,0.0,0.035

produces the levels for an "analog square" waveform.
