"""
Problema 50 — Pipeline de qualidade
Área: pandas | Nível: avançado

Enunciado:
Limpe nomes, converta valores com errors='coerce', descarte valores inválidos ou negativos, deduplique por id mantendo o último registro válido e some por cliente.

Critérios de conclusão:
O total final de Ana deve ser 15, o de Bia 20; id 3 deve ser descartado e a entrada preservada.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'id': [1, 2, 1, 3], 'cliente': [' Ana ', 'Bia', 'ANA', 'Caio'], 'valor': ['10', '20', '15', 'erro']})

# TODO: implemente a resolução deste problema.
