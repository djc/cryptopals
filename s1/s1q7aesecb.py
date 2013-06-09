from Crypto.Cipher import AES
import base64

KEY = 'YELLOW SUBMARINE'

def decrypt(cipher, key):
	aes = AES.new(key)
	return aes.decrypt(cipher)

if __name__ == '__main__':
	
	with open('s1q7-cipher.txt') as f:
		cipher = base64.b64decode(f.read())
	
	print decrypt(cipher, KEY)
