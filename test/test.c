#include <blinkmojt.h>
#include <stdio.h>
#include <unistd.h>

int main() {
    blinkmojt_t* mojt = blinkmojt_open("testmojt");

    blinkmojt_info_t info;
    blinkmojt_get_info(mojt, &info);
    printf("width: %d, height: %d, color depth: %d\n", info.width, info.height, info.depth);

    for (int i=0;;++i) {
        frame_t* frame = blinkmojt_get_frame(mojt);
        frame_set_pixel(frame, i % info.width, i / info.width % info.height, 0xF280A1FF);
        blinkmojt_draw_frame(mojt, frame);
        usleep(40000);
    }

    blinkmojt_close(mojt);
}
