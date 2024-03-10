-- factorial function
function factorial(n)
    if n == 0 then
        return 1
    else
        return n * factorial(n - 1)
    end
end

-- Example usage
local inputNumber = 5
local result = factorial(inputNumber)
print("Factorial of " .. inputNumber .. " is " .. result)