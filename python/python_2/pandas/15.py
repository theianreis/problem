"""
Problema 15 — Extração com regex
Área: pandas | Nível: básico

Enunciado:
Extraia o código numérico de cada pedido no padrão PED- seguido de três dígitos.

Critérios de conclusão:
Os códigos devem ser ['001', '120', ausente]; use str.extract.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use pandas como biblioteca principal na resolução.
"""

import pandas as pd

df = pd.DataFrame({'descricao': ['Pedido PED-001 pago', 'Pedido PED-120 aberto', 'Sem código']})

# TODO: implemente a resolução deste problema.
