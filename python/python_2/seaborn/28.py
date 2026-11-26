"""
Problema 28 — Triângulo de correlações
Área: seaborn | Nível: intermediário

Enunciado:
Oculte a diagonal e o triângulo superior de uma matriz de correlação usando mask em heatmap.

Critérios de conclusão:
Apenas as três correlações abaixo da diagonal devem ficar visíveis; mantenha escala de -1 a 1.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

import numpy as np
df = pd.DataFrame({'a': [1, 2, 3, 4, 5], 'b': [2, 1, 4, 3, 6], 'c': [5, 4, 2, 3, 1]})

# TODO: implemente a resolução deste problema.
