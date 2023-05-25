# Introduction

We chose exercise category A: compilation. As there were two of us in the group,
we decided to use imperative and object-oriented languages as our "destination" languages.
We chose Rust as the imperative language and Javascript as the object-oriented one.
Each of these programs were implemented and can be compiled as source-to-source
from MiniImp and tested on the command line with commands provided later.

The translator program is also implemented in Rust as we think it's an interesting
language and we wanted to test how this kind of task could be done with it.

The documentation is generated using [mdbook](https://github.com/rust-lang/mdBook)
with optional [mdbookPdf](https://crates.io/crates/mdbook-pdf) backend.
Docs folder contains documentation in pdf and html formats.
You can also build the documentation with mdbook.

```bash
# Build mdbook
mdbook build
# View in browser
mdbook serve --open
```

The MiniImpPlus test program can be found in project root in ```testprogram.mip```-file.
It's implementation follows the exercise description.

## Authors / group members

* Tuomas Rinne, <tumrin@utu.fi>
* Teemu Salonen, <tpsalo@utu.fi>
