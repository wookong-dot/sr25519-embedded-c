#define PUB_KEY_LEN             32
#define PRI_KEY_LEN             64
#define SIGN_LEN                64
#define BUFFER_LEN              96

#define STATUS_OK               0
#define STATUS_NOK              1
#define ERR_KEYPAIR             2
#define ERR_PRIKEY              3
#define ERR_SIGBYTE             4

typedef struct _sr_data {
	unsigned int status;
	unsigned int len;
	unsigned char data[BUFFER_LEN];
}sr_data;

void sr_init();
void sr_free(unsigned char* p);
unsigned char* sr_sign(unsigned char* message,unsigned int len,unsigned char* random,unsigned char* keypair);
unsigned char* sr_getpub(unsigned char* private_key);
unsigned char* sr_verify(unsigned char* signature, unsigned char* message,unsigned int len,unsigned char* keypair);
unsigned char* sr_keypair_from_seed(unsigned char* seed);

unsigned char* api_test(void)   __attribute__((section(".srapi")));
