#include <stdio.h>

int main() {
    auto _ = (void*)((void(*)(void))(
        [](){ 
            puts((const char*)(const int[]) {
                0x6c6c6548,
                0x77202c6f,
                0x646c726f,
                0x21
            });
        }
    ));
    
    ((void(*)(void))_)();
}
