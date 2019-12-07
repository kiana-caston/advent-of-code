def calculateFuel(mass)
  (mass / 3).round(half: :down) - 2
end

def recursiveFuel(mass)
  if calculateFuel(mass) <= 0
    return 0
  else
    return calculateFuel(mass) + recursiveFuel(calculateFuel(mass))
  end
end

masses = File.read("day1_input.txt").split
total = 0

masses.each { |mass| total += recursiveFuel(mass.to_i)}

puts total
