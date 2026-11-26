"""
Problema 47 — Janelas deslizantes
Área: numpy | Nível: avançado

Enunciado:
Use np.lib.stride_tricks.sliding_window_view para criar janelas de tamanho três e calcular o máximo de cada janela.

Critérios de conclusão:
A visão deve ter shape (4, 3) e os máximos devem ser [4, 5, 5, 9].

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use numpy como biblioteca principal na resolução.
"""

import numpy as np

valores = np.array([1, 4, 2, 5, 3, 9])
janela = 3

# TODO: implemente a resolução deste problema.
