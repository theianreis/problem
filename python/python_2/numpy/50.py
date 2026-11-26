"""
Problema 50 — Imagem em tons de cinza
Área: numpy | Nível: avançado

Enunciado:
Converta uma imagem RGB em cinza usando pesos [0.299, 0.587, 0.114], com uma operação vetorizada no último eixo.

Critérios de conclusão:
A imagem cinza deve ter shape (2, 2); o pixel branco deve valer 255 e o vermelho, aproximadamente 76.245.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use numpy como biblioteca principal na resolução.
"""

import numpy as np

imagem = np.array([[[255, 0, 0], [0, 255, 0]], [[0, 0, 255], [255, 255, 255]]], dtype=np.uint8)
pesos = np.array([0.299, 0.587, 0.114])

# TODO: implemente a resolução deste problema.
