"""
Problema 13 — Contagem de pedidos
Área: seaborn | Nível: básico

Enunciado:
Mostre a frequência de cada status com countplot em ordem definido, incluindo categorias sem observações.

Critérios de conclusão:
As barras pago, aberto e cancelado devem representar 3, 2 e 0 observações.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'status': ['pago', 'aberto', 'pago', 'pago', 'aberto']})
ordem = ['pago', 'aberto', 'cancelado']

# TODO: implemente a resolução deste problema.
