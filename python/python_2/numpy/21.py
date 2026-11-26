"""
Problema 21 — Sistema linear
Área: numpy | Nível: intermediário

Enunciado:
Resolva o sistema A x = b com np.linalg.solve e verifique o resíduo.

Critérios de conclusão:
A solução deve ser [2, 1] e np.allclose(a @ x, b) deve ser True.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use numpy como biblioteca principal na resolução.
"""

import numpy as np

a = np.array([[2., 1.], [1., -1.]])
b = np.array([5., 1.])

# TODO: implemente a resolução deste problema.
