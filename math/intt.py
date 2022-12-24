from sympy import intt

data_cols = [2, 3, 6]
prime_mod = 15*2**27 + 1
coefficients = intt(data_cols, prime_mod)

print("Result: ", coefficients)