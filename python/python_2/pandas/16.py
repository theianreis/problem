"""
Problema 16 — Contagem de categorias
Área: pandas | Nível: básico

Enunciado:
Conte as ocorrências de cada status e calcule suas proporções, incluindo valores ausentes.

Critérios de conclusão:
pago deve ter contagem 3 e proporção 0.6; aberto e ausente devem ter 1 cada.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

status = pd.Series(['pago', 'aberto', 'pago', None, 'pago'], name='status')

# TODO: implemente a resolução deste problema.
