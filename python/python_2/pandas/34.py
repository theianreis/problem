"""
Problema 34 — Calendário completo
Área: pandas | Nível: intermediário

Enunciado:
Reindexe a série para todos os dias de 2026-01-01 a 2026-01-05 e preencha dias sem vendas com zero.

Critérios de conclusão:
Os valores devem ser [10, 0, 30, 0, 50], com cinco datas.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

vendas = pd.Series([10, 30, 50], index=pd.to_datetime(['2026-01-01', '2026-01-03', '2026-01-05']))

# TODO: implemente a resolução deste problema.
