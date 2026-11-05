# `policy:set`

## Scope Relevence

## Example `policy:set`

There is a default rule for `request:rhex`, in which it accepts from anyone at a rate of X. This can be explicitly overwritten.

`o` can be omitted, and will assume the value of `k`

```rust
Policy {
    desc: "Dingus".to_string(),
    rules: [
        Rule {
            append: [
                "mingus".to_string(),
                "lingus".to_string()
            ],
            k: 3,
            o: 3, // observer count must always >= k
            quorum: [
                "quorum".to_string()
            ],
            delay: 1_000_000,
            rt: [
                "some:record".to_string()
            ],
            window: 1_000
        }
    ],
    eff: 0,
    exp: 1_000_000_000_000,
    issued: 1234,
}
```
