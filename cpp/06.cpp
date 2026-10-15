#include <iostream>

using namespace std;

int main() {
    int idade;

    cout << "Digite sua idade: ";
    cin >> idade;

    string resultado[2];
    resultado[false] = "Você é menor de idade.";
    resultado[true]  = "Você é maior de idade.";

    bool maiorDeIdade = (idade >= 18);

    cout << resultado[maiorDeIdade] << endl;

    return 0;
}