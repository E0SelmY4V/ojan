#include <iostream>
#include <cmath>

using namespace std;

double ball(double pi, double r)
{
	return (4.0 / 3.0) * pi * r * r * r;
}

int main()
{
	int id;
	cin >> id;
	double pi = 3.141593, r = 5, v = ball(pi, 4) + ball(pi, 10);
	switch (id)
	{
	case 1:
		cout << "I love Luogu!";
		break;
	case 2:
		cout << 6 << ' ' << 4;
		break;
	case 3:
		cout << 3 << endl
			 << 12 << endl
			 << 2;
		break;
	case 4:
		cout << 500.0 / 3.0;
		break;
	case 5:
		cout << 15;
		break;
	case 6:
		cout << sqrt(6.0 * 6.0 + 9.0 * 9.0);
		break;
	case 7:
		cout << 110 << endl
			 << 90 << endl
			 << 0;
		break;
	case 8:
		cout << 2 * pi * r << endl
			 << pi * r * r << endl
			 << ball(pi, r);
		break;
	case 9:
		cout << 22;
		break;
	case 10:
		cout << 9;
		break;
	case 11:
		cout << 100.0 / 3.0;
		break;
	case 12:
		cout << (int)('M' - 'A') + 1 << endl
			 << (char)('A' + 17);
		break;
	case 13:
		cout << (int)pow(v, 1.0 / 3.0);
		break;
	case 14:
		cout << 50;
		break;
	}
}
