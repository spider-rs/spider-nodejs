# Benchmarks

The speed of Spider-RS ported compared to other tools.

Spider is about 1,000x (small websites) 10,000x (medium websites), and 100,000x (production grade websites) times faster than the popular crawlee library even with the node port performance hits.

```sh
----------------------
mac Apple M1 Max
10-core CPU
64 GB of RAM memory
1 TB of SSD disk space
-----------------------
```

Test url: `https://choosealicense.com` (small)
32 pages

| `libraries`                       | `speed`               |
| :-------------------------------- | :-------------------- |
| **`spider-rs: crawl 10 samples`** | `286ms`(✅ **1.00x**) |
| **`crawlee: crawl 10 samples`**   | `1.7s` (✅ **1.00x**) |

Test url: `https://rsseau.fr` (medium)
211 pages

| `libraries`                       | `speed`               |
| :-------------------------------- | :-------------------- |
| **`spider-rs: crawl 10 samples`** | `2.5s` (✅ **1.00x**) |
| **`crawlee: crawl 10 samples`**   | `75s` (✅ **1.00x**)  |

The performance scales the larger the website and if throttling is needed.

Linux benchmarks are about 10x faster than macOS for spider-rs.
