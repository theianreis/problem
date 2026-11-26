"""
Problema 26 — Mapa de calor de vendas
Área: seaborn | Nível: intermediário

Enunciado:
Transforme os registros em uma matriz loja por mês e use heatmap com annot=True e fmt='.0f'.

Critérios de conclusão:
As linhas devem ser A e B, as colunas Jan e Fev; B/Fev deve mostrar 40.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'loja': ['A', 'A', 'B', 'B'], 'mes': ['Jan', 'Fev', 'Jan', 'Fev'], 'venda': [10, 20, 30, 40]})

# TODO: implemente a resolução deste problema.
