"""
Problema 47 — Exportação de FacetGrid
Área: seaborn | Nível: avançado

Enunciado:
Crie um displot por grupo e salve a figura resultante com o método savefig do objeto retornado.

Critérios de conclusão:
Grave 47_distribuicoes.png ao lado deste script, com dpi=150 e bbox_inches='tight', antes de show.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'valor': [1, 2, 3, 4, 5, 6, 4, 5, 6, 7, 8, 9], 'grupo': ['A'] * 6 + ['B'] * 6})

# TODO: implemente a resolução deste problema.
