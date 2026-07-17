i got inspiration from this #link("https://os.phil-opp.com")[blog]

booatloader selection:
i could just use qemu skipping to kernel but i wanted to explore about bootloaders too since im making a os, but i dont wanna spend all my time just doing repetitive stuff and dealing with legacy stuff
two realistic options only:
#link("https://crates.io/crates/bootloader_api")[bootloader_api] which is what the aforementioned blog uses
rejected because:
- don't wanna deal with manually setting up SMP
- hands me a poor memory base my mmu off of
- more niche(thereforee less mature/popular and therefore getting less support) and only gain traction due to the aforementioned blog post
  - due to that it's tightly coupled with it
#link("https://crates.io/crates/limine")[limine]
- cons: no documentation
