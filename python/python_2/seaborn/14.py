"""
Problema 14 — Contagem por canal
Área: seaborn | Nível: básico

Enunciado:
Compare quantidades de pedidos por canal e status usando countplot com hue.

Critérios de conclusão:
web deve ter dois pagos e um aberto; loja, um pago e dois abertos.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'canal': ['web'] * 3 + ['loja'] * 3, 'status': ['pago', 'pago', 'aberto', 'pago', 'aberto', 'aberto']})

# TODO: implemente a resolução deste problema.
