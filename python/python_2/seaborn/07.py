"""
Problema 07 — Histograma de tempos
Área: seaborn | Nível: básico

Enunciado:
Use histplot com bordas explícitas para mostrar a distribuição dos tempos de entrega.

Critérios de conclusão:
As quatro classes [0,5), [5,10), [10,15), [15,20] devem ter contagens 2, 3, 2 e 1.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame({'tempo': [1, 3, 5, 7, 9, 11, 14, 18]})
bordas = [0, 5, 10, 15, 20]

# TODO: implemente a resolução deste problema.
