"""
Problema 23 — Junção de pedidos e clientes
Área: pandas | Nível: intermediário

Enunciado:
Una pedidos ao cadastro pelo cliente_id, mantendo todos os pedidos e validando a relação muitos para um.

Critérios de conclusão:
O pedido com cliente_id 3 deve ficar com nome ausente; a saída deve ter três linhas.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

pedidos = pd.DataFrame({'pedido': [10, 11, 12], 'cliente_id': [1, 2, 3]})
clientes = pd.DataFrame({'cliente_id': [1, 2], 'nome': ['Ana', 'Bia']})

# TODO: implemente a resolução deste problema.
