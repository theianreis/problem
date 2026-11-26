"""
Problema 29 — Tabela de contingência
Área: pandas | Nível: intermediário

Enunciado:
Monte uma crosstab entre canal e status com totais marginais.

Critérios de conclusão:
web/pago deve ser 2, loja/aberto deve ser 1 e o total geral deve ser 4.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'canal': ['web', 'web', 'loja', 'loja'], 'status': ['pago', 'pago', 'pago', 'aberto']})

# TODO: implemente a resolução deste problema.
