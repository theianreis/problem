"""
Problema 21 — Grupos acima de uma meta
Área: pandas | Nível: intermediário

Enunciado:
Mantenha apenas os registros de clientes cujo faturamento acumulado seja pelo menos 150.

Critérios de conclusão:
Apenas as duas linhas de Ana devem permanecer, pois somam 170.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'cliente': ['Ana', 'Bia', 'Ana'], 'valor': [100, 80, 70]})
meta = 150

# TODO: implemente a resolução deste problema.
