"""
Problema 36 — Facetas de séries temporais
Área: seaborn | Nível: avançado

Enunciado:
Use relplot com kind='line', col='loja' e hue='canal' para comparar trajetórias.

Critérios de conclusão:
Cada painel deve conter duas linhas e os eixos y devem ser compartilhados.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame([(loja, canal, dia, base + dia * passo) for loja, base in [('A', 10), ('B', 20)] for canal, passo in [('web', 3), ('loja', 2)] for dia in [1, 2, 3]], columns=['loja', 'canal', 'dia', 'venda'])

# TODO: implemente a resolução deste problema.
