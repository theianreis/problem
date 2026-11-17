# Uso do match para resolução de fatorial

def fatorial(n):
    match n:
        case 0| 1: 
            return 1
        case _ if n > 1:
            return n * fatorial(n-1)
        case _ :
            raise ValueError("Fatorial não definido nesse valor")


print(fatorial(-1))
