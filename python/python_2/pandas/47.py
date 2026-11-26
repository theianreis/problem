"""
Problema 47 — Junção temporal
Área: pandas | Nível: avançado

Enunciado:
Associe cada venda à última cotação disponível no mesmo instante ou antes dele usando merge_asof.

Critérios de conclusão:
As cotações associadas devem ser [5.0, 5.2, 5.2]; ordene os dois conjuntos por horário.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

vendas = pd.DataFrame({'hora': pd.to_datetime(['2026-01-01 09:05', '2026-01-01 09:15', '2026-01-01 09:25']), 'usd': [10, 20, 30]})
cotacoes = pd.DataFrame({'hora': pd.to_datetime(['2026-01-01 09:00', '2026-01-01 09:10']), 'cotacao': [5.0, 5.2]})

# TODO: implemente a resolução deste problema.
