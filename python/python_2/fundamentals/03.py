def factorial(n):
    result = 1
    for c in range(1, n+1):
        result*=c
    return result

print(factorial(5))
