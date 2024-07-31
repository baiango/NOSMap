#include "nosmap_all.h"
#include <iostream>
using std::cout;
using std::endl;


int main() {
#ifdef TESTS
	{
	cout << "---------- Run tests ----------" << endl;
	NOSMap::test();
	cout << "---------- End of tests ----------" << endl;
	}
#endif
}
