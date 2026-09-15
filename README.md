# OS-Learning
This is a small project on how I laern to build an **Operating System** from scratch!

## Setup
This OS is made with the programming language **Rust** and uses its *bootloader* dependency to create *bootable* images.
I use several websites to learn, but the most important is **<a href="[https://](https://os.phil-opp.com)">Writing an OS in Rust</a>**

But I use youtube tutorials and other website documentation as well. If I come across a website that helped me a lot, I will add it to the **Websites Of Fame** region.

## Current Features/Progress
### Bootable
Thanks to the *bootloader* dependency, I didn't have to write an own **Bootloader**, which would have been a project on its own

### VGA-Buffer
It is the most simple and well known output at the **kernel layer** and with it, it is possible to get **ASCII** output while developing the **kernel** which is also important for *testing* features.

### Tests
Currerntly some simple tests are implemented, but the most important ones are the **VGA-Buffer** tests and **CPU Exception** tests.

### Exception Handling
I mentioned the **CPU Exception** earlier in the *Tests* feature. These can occur for several reasons, such as a **Page Fault**, but also **Breakpoint Exceptions** that are willingly used to pause a program for debugging purposes. Most of you might have used it already by making breakpoint in an IDE like vscode to debug your code. If you debugged code using this feature, you already used the *Breakpoint Exception*. If you want to know more details please visit the tab *CPU Exceptions* at the **<a href="[https://](https://os.phil-opp.com)">Writing an OS in Rust</a>** website.

## Websites Of Fame
- <a href="[https://](https://os.phil-opp.com)">Writing an OS in Rust</a>