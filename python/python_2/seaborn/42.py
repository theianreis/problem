"""
Problema 42 — Grade personalizada
Área: seaborn | Nível: avançado

Enunciado:
Crie um FacetGrid por região e use map_dataframe para aplicar scatterplot distinguindo canais por cor.

Critérios de conclusão:
As cores de web e loja devem ser consistentes em todos os painéis e a legenda deve ser única.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use seaborn como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.DataFrame([(regiao, canal, x, x * fator + base) for regiao, base in [('Sul', 0), ('Norte', 2)] for canal, fator in [('web', 2), ('loja', 3)] for x in [1, 2, 3]], columns=['regiao', 'canal', 'investimento', 'venda'])

# TODO: implemente a resolução deste problema.
