class Solution
  def initialize(noun, verb)
    @noun = noun
    @verb = verb
  end

  def runProgram(inputData)
    inputData[1] = @noun
    inputData[2] = @verb
    recursiveProgram(inputData, 0)[0]
  end

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
end

inputData = File.read("input.txt").split(',')
inputData.map! { |x| x.to_i }

noun = 1

while noun < 100
  verb = 1
  while verb < 100
    solution = Solution.new(noun, verb)
    if solution.runProgram(inputData.dup) == 19690720
      print noun, verb
      puts
      puts 100 * noun + verb
    end
    verb += 1
  end
  noun += 1
end

solution = Solution.new(12, 2)
