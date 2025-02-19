#include <iostream>

typedef unsigned long long ull;
using namespace std;

ull power(unsigned short line_ele, char now)
{
	return ((ull)line_ele) << now;
}

int main()
{
	unsigned long long sum = 0;
	int _n, _m;
	cin >> _n >> _m;
	char n = _n, m = _m;
	for (char i = 0; i < n; i++)
	{
		unsigned short line[90] = {0};
		for (char j = 0; j < m; j++)
		{
			cin >> line[j];
		}
		ull dp[90][90] = {0};
		dp[0][m] = 0;
		for (char width = m - 1; width >= 0; --width)
		{
			for (char from = 0; from <= m - width; ++from)
			{
				char pos = m - width;
				dp[from][width] = max(
					from > 0 ? dp[from - 1][width + 1] + power(line[from - 1], pos) : 0,
					from + width < m ? dp[from][width + 1] + power(line[from + width], pos) : 0);
				// cout << (int)from << ' ' << (int)width << ' ' << dp[from][width] << endl;
			}
		}
		ull max = 0;
		for (char from = 0; from < m; ++from)
		{
			if (dp[from][0] > max)
				max = dp[from][0];
		}
		sum += max;
	}
	cout << sum;
}
