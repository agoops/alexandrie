What's available ?
==================

Currently, not many options are implemented yet.

As index management strategies, we have:
- `cli`: local index clone, managed by invocations of the `git` shell command.
- **(PLANNED)** `git2`: just like `cli`, but uses `libgit2` instead of relying on the `git` shell command.
- **(PLANNED)** `remote`: remote index clone, managed by a companion server.

As crate storage strategies, we have:
- `disk`: local on-disk crate storage.
- **(PLANNED)** `remote`: just like `disk`, but on a remote machine, managed by a companion server.
- **(PLANNED)** `s3`: stores crates in an AWS S3 bucket.

**PSA:**  
The 'PLANNED' items are ideas that are possible to implement but no guarantees or deadline as to when they would actually land.  
Any help on these items are greatly welcome.  
