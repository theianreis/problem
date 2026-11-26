"""
Problema 48 — Softmax estável
Área: numpy | Nível: avançado

Enunciado:
Implemente softmax por linha, subtraindo o maior valor antes da exponenciação.

Critérios de conclusão:
A saída deve ser finita, cada linha deve somar 1 e linhas que diferem por uma constante devem ter resultados iguais.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use numpy como biblioteca principal na resolução.
"""

import numpy as np

logits = np.array([[1000., 1001., 1002.], [0., 1., 2.]])

# TODO: implemente a resolução deste problema.
