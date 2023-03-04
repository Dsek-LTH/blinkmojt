#include <blinkmojt.h>
#include <stdlib.h>
#include <stdio.h>

struct blinkmojt {
};

struct frame {
};

typedef uint32_t pixel_t;

blinkmojt_t* blinkmojt_open(char* name) {
    blinkmojt_t* mojt = malloc(sizeof(blinkmojt_t));
    printf("blinkmojt init '%s' -> %p\n", name, mojt);
    return mojt;
}

void blinkmojt_close(blinkmojt_t* mojt) {
    printf("blinkmojt %p close\n", mojt);
    free(mojt);
}

void blinkmojt_get_info(blinkmojt_t* mojt, blinkmojt_info_t* result) {
    result->width = 1;
    result->height = 2;
    result->depth = 3;
}

frame_t* blinkmojt_get_frame(blinkmojt_t* mojt) {
    frame_t* frame = malloc(sizeof(frame_t));
    printf("blinkmojt %p get frame -> %p\n", mojt, frame);
    return frame;
}

void blinkmojt_draw_frame(blinkmojt_t* mojt, frame_t* frame) {
    printf("blinkmojt %p draw frame %p\n", mojt, frame);
}

void frame_set_pixel(frame_t* frame, int x, int y, pixel_t pixel) {
    printf("frame %p set pixel (%d,%d) = %08x\n", frame, x, y, pixel);
}
