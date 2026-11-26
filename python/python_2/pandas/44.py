"""
Problema 44 — Normalização de JSON
Área: pandas | Nível: avançado

Enunciado:
Use pd.json_normalize para transformar os registros aninhados em colunas, mantendo os dados de endereço.

Critérios de conclusão:
Devem existir as colunas endereco.cidade e endereco.uf, e duas linhas.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

registros = [{'nome': 'Ana', 'endereco': {'cidade': 'Recife', 'uf': 'PE'}}, {'nome': 'Bia', 'endereco': {'cidade': 'Natal', 'uf': 'RN'}}]

# TODO: implemente a resolução deste problema.
