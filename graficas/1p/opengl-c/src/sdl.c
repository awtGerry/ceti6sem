#include <stdio.h>
#include <SDL/SDL.h>
#include <math.h>

SDL_Surface *screen = NULL;

void init()
{
    SDL_Init(SDL_INIT_EVERYTHING);
    SDL_WM_SetCaption("Bresenham", NULL);
    screen = SDL_SetVideoMode(640, 480, 32, SDL_SWSURFACE);
}

void pixel(SDL_Surface *surface, int x, int y, Uint32 pixel)
{
    int bpp = surface->format->BytesPerPixel;
    Uint8 *p = (Uint8 *)surface->pixels + y * surface->pitch + x * bpp;
    *(Uint32 *)p = pixel;
}
void line(SDL_Surface *surface, poi c1, point_t c2, Uint32 pixel)
{
    /* Bresenham's line algorithm */
    int x1 = c1.x, y1 = c1.y, x2 = c2.x, y2 = c2.y;
    int step = fabs(x2 - x1) > fabs(y2 - y1), inc = -1;

    if (step)
    {
        x1 ^= y1;
        y1 ^= x1;
        x1 ^= y1;

        x2 ^= y2;
        y2 ^= x2;
        x2 ^= y2;
    }

}
