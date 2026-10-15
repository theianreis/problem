#include <iostream>

using namespace std;

int main(){
    int array[4]= {1,2,3,4};

    for(int i: array){
        cout << i << "\n";
    }

    return 0;
}