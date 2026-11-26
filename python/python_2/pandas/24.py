"""
Problema 24 — Auditoria de junção externa
Área: pandas | Nível: intermediário

Enunciado:
Faça uma junção externa com indicator para identificar chaves presentes em apenas uma das tabelas.

Critérios de conclusão:
O id 1 deve ser left_only, 2 both e 3 right_only.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

esquerda = pd.DataFrame({'id': [1, 2], 'valor_a': [10, 20]})
direita = pd.DataFrame({'id': [2, 3], 'valor_b': [200, 300]})

# TODO: implemente a resolução deste problema.
