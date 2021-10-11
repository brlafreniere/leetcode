MAX = 2^31-1
MIN = -2^31

def reverse(x)
  reversed_x = x.to_s.reverse.to_i
  return 0 if reversed_x > MAX || reversed_x < MIN
  return reversed_x
end
