"""
Problema 43 — Tipos anuláveis
Área: pandas | Nível: avançado

Enunciado:
Converta quantidade para o tipo inteiro anulável Int64 e calcule o dobro preservando os ausentes.

Critérios de conclusão:
Quantidade e dobro devem ter dtype Int64; o ausente deve continuar pd.NA.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'quantidade': [1, None, 3]})

# TODO: implemente a resolução deste problema.
