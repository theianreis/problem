"""
Problema 49 — Sessões por inatividade
Área: pandas | Nível: avançado

Enunciado:
Ordene eventos por usuário e instante e crie novas sessões quando a pausa for estritamente maior que 30 minutos.

Critérios de conclusão:
Para A, os três eventos devem ter sessões locais [1, 1, 2]; o primeiro evento de cada usuário inicia sessão 1.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'usuario': ['A', 'A', 'A', 'B'], 'instante': pd.to_datetime(['2026-01-01 09:00', '2026-01-01 09:20', '2026-01-01 10:00', '2026-01-01 09:00'])})

# TODO: implemente a resolução deste problema.
