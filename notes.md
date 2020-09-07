# A Freestanding Rust Binary
Personal notes of Josiah on following Philipp Oppermann's blog "Writing an OS in Rust", located at https://os.phil-opp.com/

###03 Sept 2020:
- Completed *A Freestanding Rust Binary*. Got everything working so far.

###05 Sept 2020:
- Started *A Minimal Rust Kernel*
- Completed it as well
Thoughts:  
- Basic setup.
- Covers BIOS but not UEFI at this time.
- Do we run on QEMU to avoid damaging our own system?
- Could this run on a Pi? On an Arduino?
- How could this work on IoT devices?

###07 Sept 2020:
Start *VGA Text Mode*:
- VGA text buffer standard is 25 rows and 80 columns - just so, Terminal default is 80x24.
- Hmm, what do you know, it works so far.
  - Got the VGA buffer up and running in basic form.
####Semantic Versioning
https://semver.org
"Given a version number MAJOR.MINOR.PATCH, increment the:
1. MAJOR version when you make incompatible API changes,
2. MINOR version when you add functionality in a backwards compatible manner,
3. PATCH version when you make backwards compatible bug fixes.
Additional labels for pre-release and build metadata are available as
extensions to the MAJOR.MINOR.PATCH format."  

Ok, what did we learn so far?
- We accessed the VGA buffer to write to std output. This was done by direct memory access to the VGA buffer using "unsafe" Rust. For now, we compile and run via QEMU emulator to keep this off our own computer, but it should work on a bare metal system with BIOS, bootloader, and VGA buffer.
- VGA buffer has some ASCII characters, 16 colors for foreground, 8 colors for background, and blink option
- VGA prints unknown UTF-8 as two nonsense characters due to UTF-8 being 32-bit instead of 16.
