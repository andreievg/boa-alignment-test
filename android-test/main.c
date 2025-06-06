#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>

int main()
{
    void *handle = dlopen("./libboa_alignment_test.so", RTLD_LAZY);
    if (!handle)
    {
        fprintf(stderr, "Cannot load library: %s\n", dlerror());
        return 1;
    }

    int (*test_func)() = dlsym(handle, "test_boa_alignment");
    if (!test_func)
    {
        fprintf(stderr, "Cannot find symbol: %s\n", dlerror());
        dlclose(handle);
        return 1;
    }

    int result = test_func();
    dlclose(handle);
    return result;
}