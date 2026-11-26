"""
Problema 32 — Filtro entre datas
Área: pandas | Nível: intermediário

Enunciado:
Converta a coluna data e selecione registros entre 2026-01-02 e 2026-01-04 inclusive.

Critérios de conclusão:
Apenas valores 20, 30 e 40 devem permanecer.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'data': pd.date_range('2026-01-01', periods=5), 'valor': [10, 20, 30, 40, 50]})

# TODO: implemente a resolução deste problema.
