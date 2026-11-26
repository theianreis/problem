"""
Problema 36 — Acumulado por cliente
Área: pandas | Nível: avançado

Enunciado:
Ordene por cliente e data e calcule o valor acumulado individual com groupby e cumsum.

Critérios de conclusão:
Para Ana, os acumulados devem ser 10 e 40; para Bia, 20 e 60.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'cliente': ['Ana', 'Bia', 'Ana', 'Bia'], 'data': pd.to_datetime(['2026-01-01', '2026-01-01', '2026-01-02', '2026-01-02']), 'valor': [10, 20, 30, 40]})

# TODO: implemente a resolução deste problema.
