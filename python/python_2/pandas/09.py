"""
Problema 09 — Auditoria de ausentes
Área: pandas | Nível: básico

Enunciado:
Conte valores ausentes por coluna e calcule sua proporção percentual.

Critérios de conclusão:
idade deve ter um ausente e cidade, dois; informe percentuais aproximadamente 33.33 e 66.67.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'nome': ['Ana', 'Bia', 'Caio'], 'idade': [20, None, 30], 'cidade': [None, 'Natal', None]})

# TODO: implemente a resolução deste problema.
