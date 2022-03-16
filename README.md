# hash-collisions

Simple hello world test to see how likely crc32 and other hashing algorithms are to collide on simple utf-8 inputs

From the hashing algorithms tested (MD5, SHA1, SHA256, CRC32, CRC64, various fasthash and Rust default hashers), only CRC32 had a practical collision, and it actually had a lot:

```
Collisions by algorithm:
  Crc32: 1373 total collided hashes, 2746 words
```

Food for thought I suppose :)
