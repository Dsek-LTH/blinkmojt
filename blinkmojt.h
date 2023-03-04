/* Blinkmojt Standard API */

#ifndef __BLINKMOJT_H__
#define __BLINKMOJT_H__

#include <stdint.h>

typedef struct blinkmojt blinkmojt_t;
typedef struct frame frame_t;
typedef uint32_t pixel_t;

typedef struct blinkmojt_info {
    int width;
    int height;
    int depth;
} blinkmojt_info_t;

blinkmojt_t* blinkmojt_open(char* name);
void blinkmojt_close(blinkmojt_t* mojt);
void blinkmojt_get_info(blinkmojt_t* mojt, blinkmojt_info_t* result);

frame_t* blinkmojt_get_frame(blinkmojt_t* mojt);
void blinkmojt_draw_frame(blinkmojt_t* mojt, frame_t* frame);

void frame_set_pixel(frame_t* frame, int x, int y, pixel_t pixel);

#endif // __BLINKMOJT_H__
