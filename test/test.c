#include <blinkmojt.h>
#include <stdio.h>

int main() {
    blinkmojt_t* mojt = blinkmojt_open("testmojt");

    blinkmojt_info_t info;
    blinkmojt_get_info(mojt, &info);
    printf("width: %d, height: %d, color depth: %d\n", info.width, info.height, info.depth);

    frame_t* frame = blinkmojt_get_frame(mojt);
    frame_set_pixel(frame, 0, 0, 0xF280A100);
    blinkmojt_draw_frame(mojt, frame);

    blinkmojt_close(mojt);
}