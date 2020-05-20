import sys

def pad(s, n):
	assert n < 256 and len(s) < n
	return s + ''.join(chr(n - len(s)) for i in range(n - len(s)))

if __name__ == '__main__':
	print repr(pad(sys.argv[1], int(sys.argv[2])))
