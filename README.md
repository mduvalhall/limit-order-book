# Limit Order Book

A limit order book implemented in three languages.

## Implementations

| Folder               | Language | Purpose                                  | Docs                                 |
| -------------------- | -------- | ---------------------------------------- | ------------------------------------ |
| [`python/`](python/) | Python   | POC: simple logic to validate the design | [python/README.md](python/README.md) |
| [`rust/`](rust/)     | Rust     | Performant implementation                | [rust/README.md](rust/README.md)     |
| [`cpp/`](cpp/)       | C++      | Performant implementation                | [cpp/README.md](cpp/README.md)       |

## Structure

```
limit-order-book/
├── python/
├── rust/
└── cpp/
```

## Getting Started

Each implementation has its own build and run instructions. See the README in the relevant folder.

## Notes

- The Python POC prioritises clarity over performance. It is the reference for the intended behaviour.
- The Rust and C++ versions follow the same core behaviour as the POC.
- Use of AI is limited to assitance with writing READMEs and other documentation.
