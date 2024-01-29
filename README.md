# Brim-to-Mint

## Usage

download your transaction from the BRIM website

run the rust program and give it the name of your CSV file

the output will be in Mint format which Quicken can import


```rust
    cargo run -- path_to_csv_file.csv
```

## Older Code

there is also an older ruby script that does some thing similar

```ruby
    ruby main.rb path_to_csv_file.csv > brim.qif
```
