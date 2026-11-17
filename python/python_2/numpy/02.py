# Criação de array randomizada

import numpy as np

matriz_randomizada = np.random.rand(3, 2)
matriz_inteiros = np.random.randint(low=1, high=100, size=(3, 3))

print(f"Matriz randomizada: \n{matriz_randomizada} \n Matriz Interira: \n{matriz_inteiros}")