inputData = File.read("input.txt").split(',')
inputData.map! { |x| x.to_i }

mainInput = inputData
mainInput[1] = 12
mainInput[2] = 2

def recursiveProgram(input, position)
  opcode = input[position]
  arrayIndices = input[position + 1, 3]

  if opcode == 99
    return input
  else
    input[arrayIndices[2]] = calculateCode(opcode, input[arrayIndices[0]], input[arrayIndices[1]])
    recursiveProgram(input, position + 4)
  end
end

def calculateCode(opcode, a, b)
  if opcode == 1
    return a + b
  elsif opcode == 2
    return a * b
  else
    puts "error"
  end
end

puts recursiveProgram(mainInput, 0)[0]
