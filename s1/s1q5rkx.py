import base64

PLAIN = ''.join((
	"Burning 'em, if you ain't quick and nimble\n",
	"I go crazy when I hear a cymbal",
))
KEY = 'ICE'
CIPHER = ''.join((
	'0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a2622632427276527',
	'2a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f',
))

def encrypt(plain, key):
	
	cipher = []
	key = [ord(c) for c in key]
	for i, c in enumerate(plain):
		cipher.append(chr(key[i % len(key)] ^ ord(c)))
	
	return ''.join(cipher)

if __name__ == '__main__':
	cipher = encrypt(PLAIN, KEY)
	print base64.b16encode(cipher).lower() == CIPHER
