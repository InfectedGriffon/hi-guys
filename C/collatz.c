#include <stdlib.h>
#include <stdio.h>

void printBits(int g){
  for(int i=31;i>=0;i--){
    printf("%d",((1<<i)&g)>>i);
  }
printf("\n");
}

enum f2Opts{
  sevens1,
  sevens3,
  base3
};

void collatz(int n){
    while(1&~n)
        n>>=1;
    if(n==1) return;
    printBits(n);
    printf("%d\n",n);
    collatz(n*3+1);
}

int f1(int n){
  int t=0;
  for(int i=0;i<n*2;i+=2)
    t|=1<<i;
  return t;
}

int f2(int a, int b, int c){
  int t=0;
  switch(c){
  case 0:
    for(int i=0;i<a;i++)
      t|=1<<(i*6);
    t*=112;
    t++;
    break;
  case 1:
    for(int i=0;i<a;i++)
      t|=1<<(i*6);
    t*=224;
    t+=3;
    break;
  case 2:
    t=3;
  }
  t<<=2*b;
  t|=f1(b);
  return t;
}

int main(){
    collatz(f2(2,2,sevens1));
}