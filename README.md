# Brim-to-Mint

## Overview
This tool will convert a CSV of credit card transactions from Brim Financial into a format that Quicken can import.

Brim Financial does not support downloading your credit card transactions directly into Quicken.  They do allow you to download the transactions in their own proprietary CSV format though.  This tool will convert the CSV into the format used by Mint, which Quicken can import. You can then use the import menu item to bring in the transactions.  

## Usage

1. download your transaction from the BRIM website
1. run the program and give it the name of your CSV file (see details below)
1. the output will be in Mint format which Quicken can import
1. go to Quicken and use the import menu to bring the transactions into a new account
1. drag the transaction from the new account to the account you want them in

```rust
    cargo run -- path_to_csv_file.csv
```

## Issues

1. Quicken will default the amounts to be positive (debits) rather than credits.  I've tried to get the import to respect if it a debit or credit but with no luck.  I'm not sure if its a bug of Quicken or something in the code for this tool.  Once they are in Quicken, you will have to manually change them from a debit to a credit.

## Older Code
there is also an older ruby script that does some thing similar

```ruby
    ruby main.rb path_to_csv_file.csv > brim.qif
```
