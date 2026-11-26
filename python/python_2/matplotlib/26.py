"""
Problema 26 — Datas no eixo x
Área: matplotlib | Nível: intermediário

Enunciado:
Use objetos datetime para desenhar medições diárias e formate ticks com dia e mês.

Critérios de conclusão:
Utilize matplotlib.dates e evite sobreposição dos rótulos com autofmt_xdate ou rotação.

Instruções:
Use os dados iniciais abaixo e implemente a resolução após o TODO.
Use matplotlib como biblioteca principal na resolução.
Inclua título, rótulos dos eixos e finalize com plt.show().
"""

import matplotlib.pyplot as plt

from datetime import datetime, timedelta
datas = [datetime(2026, 1, 1) + timedelta(days=i) for i in range(10)]
valores = [10, 12, 11, 15, 14, 18, 17, 20, 19, 22]

# TODO: implemente a resolução deste problema.
