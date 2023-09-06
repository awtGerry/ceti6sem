#include <bits/stdc++.h>
using namespace std;
 
// function for line generation
void bresenham(int x1, int y1, int x2, int y2)
{
    int m_new = 2 * (y2 - y1);
    int slope_error_new = m_new - (x2 - x1);
    for (int x = x1, y = y1; x <= x2; x++) {
        cout << "(" << x << "," << y << ")\n";
 
        slope_error_new += m_new;
 
        if (slope_error_new >= 0) {
            y++;
            slope_error_new -= 2 * (x2 - x1);
        }
    }
}

int main()
{
    int x1 = 30, y1 = 20, x2 = 40, y2 = 36;
    bresenham(x1, y1, x2, y2);
    return 0;
}
