total = 0

File.foreach("input.txt") { |mass|
  total += (mass.to_i / 3).round(half: :down) - 2
}

puts total
