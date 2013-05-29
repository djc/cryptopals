import base64

SOURCE = ''.join((
	'49276d206b696c6c696e6720796f757220627261696e206c',
	'696b65206120706f69736f6e6f7573206d757368726f6f6d',
))

TARGET = 'SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t'

def b16decode(s):
	return base64.b16decode(s, True)

def b64encode(s):
	return base64.b64encode(s)

if __name__ == '__main__':
	print b16decode(SOURCE)
	print TARGET
	print b64encode(b16decode(SOURCE)) == TARGET
