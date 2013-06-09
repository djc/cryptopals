import base64

SOURCE = base64.b16decode('1c0111001f010100061a024b53535009181c', True)
KEY = base64.b16decode('686974207468652062756c6c277320657965', True)
TARGET = base64.b16decode('746865206b696420646f6e277420706c6179', True)

def xor(x, y):
	assert len(x) == len(y)
	return ''.join(chr(ord(c) ^ ord(k)) for (c, k) in zip(x, y))

if __name__ == '__main__':
	print xor(SOURCE, KEY) == TARGET
