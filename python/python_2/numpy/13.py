"""
Problema 13 — Broadcasting de preços
Área: numpy | Nível: básico

Enunciado:
Aplique a cada linha de preços os descontos de cada coluna, sem laços Python.

Critérios de conclusão:
A primeira linha deve virar [90, 160, 210] e o resultado deve ter shape (2, 3).

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use numpy como biblioteca principal na resolução.
"""

import numpy as np

precos = np.array([[100., 200., 300.], [50., 80., 120.]])
descontos = np.array([0.1, 0.2, 0.3])

# TODO: implemente a resolução deste problema.
