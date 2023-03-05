import socket
import random
import pygame
import math
import sys
from pygame.locals import *

pygame.init()

hugo_mode = len(sys.argv) > 1 and sys.argv[1] == "--hugo"

if hugo_mode:
    LED_SIZE = 16
    LED_MARGIN = 2
    LED_BOX = LED_MARGIN * 2 + LED_SIZE

    WIDTH = 95
    HEIGHT = 7

    LEVELS = 10

    WINDOW_SIZE = (WIDTH * LED_BOX, HEIGHT * LED_BOX)

    display = pygame.display.set_mode(WINDOW_SIZE)
    pygame.display.set_caption("Blinkmojt Simulator 2022")


    def draw_led(x, y, image):
        intensity = image[y*WIDTH + x]
        intensity = math.floor(intensity / (255 / LEVELS)) / LEVELS;
        cx = (x + 0.5)*LED_BOX
        cy = (y + 0.5)*LED_BOX
        pygame.draw.circle(surface=display, center=(cx,cy), color=(128 + (255-128)*intensity if intensity > 0 else 64, 0, 0), radius=LED_SIZE/2)

    image = [int(random.random() * 256) for i in range(WIDTH*HEIGHT)]

else:
    LED_SIZE = 8
    LED_MARGIN = 1
    LED_BOX = LED_MARGIN * 2 + LED_SIZE

    WIDTH = 64
    HEIGHT = 32

    WINDOW_SIZE = (WIDTH * LED_BOX, HEIGHT * LED_BOX)

    display = pygame.display.set_mode(WINDOW_SIZE)
    pygame.display.set_caption("Blinkmojt Simulator 2022")


    def draw_led(x, y, image):
        cx = (x + 0.5)*LED_BOX
        cy = (y + 0.5)*LED_BOX
        i = (y*WIDTH + x)*4
        r = image[i+0]
        g = image[i+1]
        b = image[i+2]
        pygame.draw.circle(surface=display, center=(cx,cy), color=(r,g,b), radius=LED_SIZE/2)

    image = [int(random.random() * 256) for i in range(WIDTH*HEIGHT*4)]


def draw_frame(image):
    for x in range(WIDTH):
        for y in range(HEIGHT):
            draw_led(x, y, image)

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.settimeout(0.1)
sock.bind(("0.0.0.0", 1337))

running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    display.fill((0,0,0))
    draw_frame(image)

    pygame.display.update()

    try:
        expected_size = WIDTH*HEIGHT if hugo_mode else WIDTH*HEIGHT*4
        buf = sock.recv(expected_size)
        if len(buf) == expected_size:
            image = buf
        else:
            print("malformed message of length ", len(buf))
    except socket.timeout:
        pass

