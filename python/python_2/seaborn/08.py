"""
Problema 08 — Histogramas por canal
Área: seaborn | Nível: básico

Enunciado:
Compare tempos por canal em histogramas com hue, bins compartilhados e stat='density'.

Critérios de conclusão:
Use common_norm=False para normalizar cada grupo separadamente; mantenha transparência.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'tempo': [1, 2, 3, 4, 5, 6, 4, 5, 6, 7, 8, 9], 'canal': ['web'] * 6 + ['loja'] * 6})

# TODO: implemente a resolução deste problema.
