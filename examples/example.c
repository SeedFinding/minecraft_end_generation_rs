#include "../target/minecraft_end_gen_rs.h"
#include <stdio.h>

// gcc -L../target/release -Wl,-rpath=../target/release -o example example.c -lminecraft_end_gen_rs
// embed the rpath inside the executable, please make sure to be right when doing that
int main(){
    EndGen* endGen=create_new_end(1551515151585454LL);
    EndBiomes biome=get_biome(endGen,10000,251,10000);
    if (biome==SmallEndIslands){
        printf("That's a win\n");
    }
    printf("%llu\n",sizeof (endGen->seed));
    printf("%llu\n",sizeof (endGen->_noise));
    delete(endGen);
}
