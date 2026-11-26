"""
Problema 33 — Agregação mensal
Área: pandas | Nível: intermediário

Enunciado:
Use um DatetimeIndex e resample com frequência de início de mês para somar as vendas.

Critérios de conclusão:
Janeiro deve somar 30 e fevereiro 70; os rótulos devem ser 2026-01-01 e 2026-02-01.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'data': pd.to_datetime(['2026-01-01', '2026-01-20', '2026-02-02', '2026-02-15']), 'venda': [10, 20, 30, 40]})

# TODO: implemente a resolução deste problema.
