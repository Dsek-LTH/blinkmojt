#include <blinkmojt.h>
#include <stdlib.h>
#include <stdio.h>

struct frame {
    blinkmojt_t* mojt;
    uint8_t* buffer;
};

struct blinkmojt {
    int width;
    int height;
    frame_t frame;
};

typedef uint32_t pixel_t;

blinkmojt_t* blinkmojt_open(char* name) {
    int width = 64;
    int height = 64;

    char* ptr;
    if (ptr = getenv("BLINK_WIDTH")) {
        width = atoi(ptr);
    }
    if (ptr = getenv("BLINK_HEIGHT")) {
        height = atoi(ptr);
    }

    blinkmojt_t* mojt = malloc(sizeof(blinkmojt_t));
    mojt->width = width;
    mojt->height = height;
    mojt->frame.buffer = malloc(sizeof(uint8_t) * width * height);
    mojt->frame.mojt = mojt;
    return mojt;
}

void blinkmojt_close(blinkmojt_t* mojt) {
    free(mojt->frame.buffer);
    free(mojt);
}

void blinkmojt_get_info(blinkmojt_t* mojt, blinkmojt_info_t* result) {
    result->width = mojt->width;
    result->height = mojt->height;
    result->depth = 1;
}

frame_t* blinkmojt_get_frame(blinkmojt_t* mojt) {
    return &mojt->frame;
}

void blinkmojt_draw_frame(blinkmojt_t* mojt, frame_t* frame) {
    printf("\e[2J");
    for (int y=0; y<mojt->height; ++y) {
        for (int x=0; x<mojt->width; ++x) {
            if (frame->buffer[y * mojt->height + x] == 0) {
                putc(' ', stdout);
            }
            else {
                putc('#', stdout);
            }
            frame->buffer[y * mojt->height + x] = 0;
        }
        putc('\n', stdout);
    }
    fflush(stdout);
}

void frame_set_pixel(frame_t* frame, int x, int y, pixel_t pixel) {
    if (x < 0 || x >= frame->mojt->width || y < 0 || y >= frame->mojt->height) {
        return;
    }

    frame->buffer[y * frame->mojt->height + x] = (uint8_t)pixel;
}
