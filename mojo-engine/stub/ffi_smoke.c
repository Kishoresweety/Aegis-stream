// Calls the stub against a real .bin; corrupt magic must yield VERDICT_ERROR.
#include "mojoguard.h"
#include <stdio.h>
#include <stdlib.h>
int main(int argc, char** argv) {
    if (argc < 2) return 2;
    FILE* f = fopen(argv[1], "rb"); if (!f) return 2;
    fseek(f, 0, SEEK_END); long n = ftell(f); fseek(f, 0, SEEK_SET);
    uint8_t* p = aligned_alloc(64, ((n + 63) / 64) * 64);
    if (fread(p, 1, n, f) != (size_t)n) return 2;
    uint8_t w[] = "hello 4111-11";
    float s = -1; uint32_t r = 99;
    int32_t v = mojo_inspect_window(w, sizeof w - 1, 0, p, &s, &r);
    printf("verdict=%d score=%.2f rule=%u header_size=%zu\n", v, s, r, sizeof(AegisPolicyHeader));
    p[0] ^= 0xFF;
    int32_t bad = mojo_inspect_window(w, sizeof w - 1, 0, p, &s, &r);
    printf("corrupt-magic verdict=%d\n", bad);
    return (v == VERDICT_PASS && bad == VERDICT_ERROR && sizeof(AegisPolicyHeader) == 64) ? 0 : 1;
}
