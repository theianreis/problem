"""
Problema 48 — Quantis em letter-value plot
Área: seaborn | Nível: avançado

Enunciado:
Use boxenplot para comparar distribuições de três grupos com 200 observações sintéticas locais por grupo.

Critérios de conclusão:
Mostre três categorias e explique em comentário como as caixas adicionais detalham as caudas.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame([(grupo, base + (i % 29) + (i % 7) * 0.3) for grupo, base in [('A', 10), ('B', 20), ('C', 30)] for i in range(200)], columns=['grupo', 'valor'])

# TODO: implemente a resolução deste problema.
