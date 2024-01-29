#!/usr/bin/env ruby
# -*- coding: utf-8 -*-

# require 'debug'

require 'csv'
# require 'iconv'
# require 'parsedate'

# get the name of the csv file
filename = String(ARGV[0])
raise "File %s not found" % filename if not File.exist? filename

# Convert LATIN1 File to UTF8
csv_data = File.open(filename).read 
# utf8_csv = Iconv.iconv("UTF-8", "LATIN1", latin_csv).join
reader = CSV.new(csv_data) 

default_header = ['No', 'Transaction Date', 'Posted Date', 'Description', 'Cardholder', 'Amount', 'Points', 'Category']
# default_header = ['No', 'Transaction Date', 'Posted Date', 'Description', 'Cardholder', 'Amount', 'Points', 'Category', 'Last 4 Digits']

# 9.times{reader.shift}

header = reader.shift
raise StandardError, "No Header found" if header != default_header

# No,"Transaction Date","Posted Date",Description,Cardholder,Amount,Points,Category,"Last 4 Digits"

puts "!Type:CCard"

for row in reader do
  date      = row[1].split('-')
  date      = sprintf("D%s\/%s\/%s", date[1], date[2], date[0])
  payee      = sprintf("M%s", row[3])
  amount    = sprintf("T%.2f", row[5])
  category  = sprintf("N%s", row[7])
  # clearance = 'C*'

  puts date, payee, amount, category, '^'
end