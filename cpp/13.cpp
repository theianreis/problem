#include <iostream>

using namespace std;

void insertionSort(int A[], int n){
    for (int j=1; j<n;j++){
        int chave = A[j];
        int i = j -1;

        while (i>=0 && A[i]>chave)
        {
            A[i+1] = A[i];
            i=i-1;
        }
        A[i+1]= chave;
    }
}

int main(){
    int A[]= {5,2,4,6,1,8};
    int n = 6;

    insertionSort(A,n);

    for(int i= 0; i<n;i++){
        cout<<A[i]<< ",";
    }

    cout<< endl;

    return 0;
}