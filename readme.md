# Cargo Rust project base  

This is template structure for cargo projects

## Dev setup  

For execution app on save, use command:  

``` sh
cargo watch -q -c -w src/ -w .cargo/ -x run
```

For execution test app on save, use command:  

```sh
cargo watch -q -c -w examples/ -x "run --example quick-dev"
```
