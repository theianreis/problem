"""
Problema 12 — Deduplicação de cadastro
Área: pandas | Nível: básico

Enunciado:
Remova duplicatas por email, preservando o registro com atualização mais recente.

Critérios de conclusão:
Para a@x.com, mantenha o nome Ana Maria; o cadastro final deve ter duas linhas.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'email': ['a@x.com', 'b@x.com', 'a@x.com'], 'nome': ['Ana', 'Bia', 'Ana Maria'], 'atualizacao': ['2026-01-01', '2026-01-02', '2026-01-03']})

# TODO: implemente a resolução deste problema.
