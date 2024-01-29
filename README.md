# Brim-to-QIF

## Usage

download your transaction from the BRIM website

run the rust program and give it the name of your CSV file

```rust
    cargo run -- path_to_csv_file.csv
```

there is also an older ruby script that does some thing similar

```ruby
    ruby main.rb path_to_csv_file.csv > brim.qif
```
