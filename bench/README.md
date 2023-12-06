# Benchmarks

```sh
Linux
8-core CPU
32 GB of RAM memory
-----------------------
```

Test url: `https://choosealicense.com` (small)
32 pages

| `libraries`                       | `speed` |
| :-------------------------------- | :------ |
| **`spider-rs: crawl 10 samples`** | `76ms`  |
| **`crawlee: crawl 10 samples`**   | `1s`    |

Test url: `https://rsseau.fr` (medium)
211 pages

| `libraries`                       | `speed` |
| :-------------------------------- | :------ |
| **`spider-rs: crawl 10 samples`** | `0.5s`  |
| **`crawlee: crawl 10 samples`**   | `72s`   |

```sh
----------------------
mac Apple M1 Max
10-core CPU
64 GB of RAM memory
-----------------------
```

Test url: `https://choosealicense.com` (small)
32 pages

| `libraries`                       | `speed` |
| :-------------------------------- | :------ |
| **`spider-rs: crawl 10 samples`** | `286ms` |
| **`crawlee: crawl 10 samples`**   | `1.7s`  |

Test url: `https://rsseau.fr` (medium)
211 pages

| `libraries`                       | `speed` |
| :-------------------------------- | :------ |
| **`spider-rs: crawl 10 samples`** | `2.5s`  |
| **`crawlee: crawl 10 samples`**   | `75s`   |

The performance scales the larger the website and if throttling is needed. Linux benchmarks are about 10x faster than macOS for spider-rs.
