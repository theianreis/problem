"""
Problema 45 — CSV em memória
Área: pandas | Nível: avançado

Enunciado:
Leia o CSV separado por ponto e vírgula, interpretando vírgulas decimais e datas no formato brasileiro.

Critérios de conclusão:
O total da coluna valor deve ser 31.25 e a coluna data deve ter dtype datetime; use io.StringIO.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

conteudo = chr(10).join(['data;valor', '01/02/2026;10,50', '02/02/2026;20,75'])

# TODO: implemente a resolução deste problema.
