#include<stdio.h>
#include<graphics.h>

void drawline(int x1, int y1, int x2, int y2)
{
    int dx, dy;
    int k, x, y;
    dx = x2 - x1;
    dy = y2 - y1;
    k = 2 * dy - dx;

    x = x1;
    y = y1;

    while(x < x2)
    {
        if(k >= 0)
        {
            putpixel(x, y, 7);
            y+=1;
            k = k + 2 * dy - 2 * dx;
        }
        else
        {
            putpixel(x, y, 7);
            k = k + 2 * dy;
        }
        x+=1;
    }
}


int main()
{
    int gd = DETECT, gm, error, x1, y1, x2, y2;
    initgraph(&gd, &gm, "C:\\TC\\BGI");
    printf("Enter co-ordinates of first point: ");
    scanf("%d%d", &x1, &y1);
    printf("Enter co-ordinates of second point: ");
    scanf("%d%d", &x2, &y2);
    drawline(x1, y1, x2, y2);
    return 0;
}
