import numpy as np

# Criando uma matriz original (3x4)
matriz = np.array([
    [1, 2, 3, 4],
    [5, 6, 7, 8],
    [9, 10, 11, 12]
])

# 1. Usando o atributo .T (forma mais rápida)
transposta_t = matriz.T

# 2. Usando a função np.transpose()
transposta_func = np.transpose(matriz)

print("Matriz Original (3x4):")
print(matriz)

print("\nMatriz Transposta (4x3):")
print(transposta_t)