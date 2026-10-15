#include <iostream>

using namespace std;

int main(){
    float nota1,nota2 ,nota3,nota4;
    float media;

    cout << "Digite a nota 1: ";
    cin >> nota1;
    cout << "Digite a nota 2: ";
    cin >> nota2;
    cout << "Digite a nota 3: ";
    cin >> nota3;
    cout << "Digite a nota 4: ";
    cin >> nota4;


    media = (nota1+nota2+nota3+nota4)/4;

    cout << "A média é das nota é de: " << media;

    if(media < 7){
        cout<< "\n Desaprovado";
    }else{
        cout<< "\n Aprovado";
    }

    return 0;
}