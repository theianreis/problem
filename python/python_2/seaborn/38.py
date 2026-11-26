"""
Problema 38 — Facetas categóricas
Área: seaborn | Nível: avançado

Enunciado:
Use catplot com kind='box', x='turno', y='nota' e col='turma'.

Critérios de conclusão:
Mostre duas caixas por turma e mantenha a ordem manhã, tarde em ambos os painéis.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame([(turma, turno, base + i) for turma, base in [('A', 4), ('B', 5)] for turno in ['manhã', 'tarde'] for i in [1, 2, 3]], columns=['turma', 'turno', 'nota'])

# TODO: implemente a resolução deste problema.
