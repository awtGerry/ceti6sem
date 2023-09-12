#include <stdio.h>
#include <stdlib.h>
#include <graphics.h>

void main(void)
{
    int gd = DETECT, gm;
    int x1, y1, x2, y2, dx, dy, steps, i;
    initgraph(&gd, &gm, NULL);
    printf("Enter the value of x1 and y1 : ");
    scanf("%d %d", &x1, &y1);
    printf("Enter the value of x2 and y2: ");
    scanf("%d %d", &x2, &y2);

}
