# hash-collisions

Simple hello world test to see how likely crc32 and other hashing algorithms are to collide on simple utf-8 inputs

From the hashing algorithms tested (MD5, SHA1, SHA256, CRC32, CRC64, various fasthash and Rust default hashers), only CRC32 had a practical collision, and it actually had a lot:

```
$ cargo run -- ./HugeWordList.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/hash-collisions ./HugeWordList.txt`
Collisions by algorithm:
  Crc32     : 1373 total collided hashes, 2746 words
  Md5       : 0 total collided hashes, 0 words
  Crc64     : 0 total collided hashes, 0 words
  Spooky64  : 0 total collided hashes, 0 words
  Metro64   : 0 total collided hashes, 0 words
  StdDefault: 0 total collided hashes, 0 words
```

My test input was some brute force password database I found online, but you
can also test with `/usr/share/dict/words` which I used in the early days to
make sure it all worked.  CRC32 also collided a few times with that much
smaller database as well.

Food for thought I suppose :)
