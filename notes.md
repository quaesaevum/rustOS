# A Freestanding Rust Binary
Personal notes of Josiah on following Philipp Oppermann's blog "Writing an OS in Rust", located at https://os.phil-opp.com/

### 03 Sept 2020:
- Completed *A Freestanding Rust Binary*. Got everything working so far.

### 05 Sept 2020:
- Started *A Minimal Rust Kernel*
- Completed it as well
Thoughts:  
- Basic setup.
- Covers BIOS but not UEFI at this time.
- Do we run on QEMU to avoid damaging our own system?
- Could this run on a Pi? On an Arduino?
- How could this work on IoT devices?

### 07 Sept 2020:
Start *VGA Text Mode*:
- VGA text buffer standard is 25 rows and 80 columns - just so, Terminal default is 80x24.
- Hmm, what do you know, it works so far.
  - Got the VGA buffer up and running in basic form.
#### Semantic Versioning
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
- VGA characters are 16-bit: 8 bits for character, 4 bits foreground color, 3 bits background color, 1 bit to toggle blink
- VGA prints unknown UTF-8 as two nonsense characters due to UTF-8 being 32-bit instead of 16.

### 08 Sept 2020:
Continuing *VGA Text Mode*:  
Finished.  
What did we learn?  
- Stuff about writing macros

#### Started *Testing*
https://wiki.osdev.org  
Built a basic trivial test without std:
- Had to set up our own system of testing that mimics normal std testing
- Built a unique set of exit codes to avoid confusion with QEMU's exit codes
Completed up to *Printing to the Console* subheading  

### 09 Sept 2020:
Continuing *Testing* from subheading *Printing to the Console*:
- lazy_static provides a macro that defines a static when it is used the first time rather than at compile time as is normal for statics. This allows arbitrarily complex initialization code as well.  

### 18 Sept 2020:
Had some time in course work, so I was away from this project for a bit.
- Finished *CPU exceptions*.
  - Good first read of lots of topics, but much more here that I don't know.
  - May return fruitfully in another couple of months of related study so that I understand more.

### 19 Sept 2020:
Back at it again.
Start *Double Fault Exceptions*.
- Occurs when CPU fails to invoke an exception handler.
  - very important! If fail at double fault, a triple fault is FATAL!
    - Most hardware reacts with system reset at triple fault.
- added a double fault handler. this sets initial ability to catch all rest of faults, since any double fault is now caught
