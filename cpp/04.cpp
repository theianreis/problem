#include <iostream>
#include <cstdio>

using namespace std;

int main(){
    int a = 1;
    int b = 2;

    int t = a;
    
    a = b;
    b = t;


    printf("%d, %d", a,b);

    return 0;
}