"""
Problema 46 — Exportação e recuperação
Área: pandas | Nível: avançado

Enunciado:
Exporte o DataFrame para uma string CSV sem índice e recupere-o com read_csv e io.StringIO.

Critérios de conclusão:
Use pd.testing.assert_frame_equal para conferir a recuperação, incluindo os nomes das colunas.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'produto': ['café', 'leite'], 'quantidade': [2, 3], 'valor': [18.5, 5.0]})

# TODO: implemente a resolução deste problema.
