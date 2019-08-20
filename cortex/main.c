#include <string.h>
#include <stdlib.h>
#include "libsr.h"
#include "sr_test.h"


unsigned int __sheap = 0x20000400;
unsigned char rv = 0;
unsigned char* u8 = 0;

void simple_test()
{
	unsigned char *p1 =  0;
	//unsigned char *p2 =  0;
	unsigned char mes[96] = { 0 };
	unsigned char mes2[96] = { 0 };
	memset(mes,0x56,96);
	memset(mes2,0x78,96);
	sr_init();
	//p1 = test_box(mes);
	//sr_free(p1);
	//p2 = test_box(mes2);
	//sr_free(p2);
	//rv = test_sign_verify();
	api_test();
}


void main(void)
{
	//simple_test();
	u8 = api_test();
	
	while(1)
	{

	}
}