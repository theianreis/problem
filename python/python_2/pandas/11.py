"""
Problema 11 — Remoção de registros incompletos
Área: pandas | Nível: básico

Enunciado:
Remova linhas sem cliente ou valor, mas mantenha linhas sem observação.

Critérios de conclusão:
Apenas os pedidos 1 e 4 devem permanecer.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'pedido': [1, 2, 3, 4], 'cliente': ['Ana', None, 'Caio', 'Duda'], 'valor': [10, 20, None, 40], 'observacao': [None, 'ok', 'ok', None]})

# TODO: implemente a resolução deste problema.
