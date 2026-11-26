"""
Problema 07 — Ordenação com desempate
Área: pandas | Nível: básico

Enunciado:
Ordene pedidos por valor decrescente e por cliente crescente em caso de empate.

Critérios de conclusão:
A ordem dos clientes deve ser ['Ana', 'Caio', 'Bia']; redefina o índice começando em zero.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'cliente': ['Caio', 'Bia', 'Ana'], 'valor': [100, 50, 100]})

# TODO: implemente a resolução deste problema.
