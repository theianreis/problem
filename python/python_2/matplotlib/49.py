"""
Problema 49 — Atualização com slider
Área: matplotlib | Nível: avançado

Enunciado:
Trace y = a*x² e adicione um Slider para alterar a de 0.5 a 3.0 sem recriar a figura.

Critérios de conclusão:
O callback deve atualizar a linha com set_ydata e draw_idle; teste em um backend interativo local.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use matplotlib como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import matplotlib.pyplot as plt

import numpy as np
from matplotlib.widgets import Slider
x = np.linspace(-2, 2, 100)
a_inicial = 1.0

# TODO: implemente a resolução deste problema.
