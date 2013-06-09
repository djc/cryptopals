import base64, s1q3break, itertools

CIPHER_FILE = 's1q6-cipher.txt'

def hamming(x, y):
	
	assert len(x) == len(y)
	dist = 0
	for p, q in zip(x, y):
		for i in range(8):
			if ord(p) & 2 ** i ^ ord(q) & 2 ** i:
				dist += 1
	
	return dist

def findsize(cipher):
	
	res = {}
	for size in range(2, 40):
		
		vals = []
		chunks = [cipher[i * size:(i + 1) * size] for i in range(4)]
		for pair in itertools.combinations(chunks, 2):
			vals.append(hamming(*pair) / float(size))
		
		res[size] = sum(vals) / len(vals)
	
	ranked = sorted((sz, val) for (val, sz) in res.iteritems())
	return ranked[0][1]

def transpose(cipher, size):
	buffers = [list() for i in range(size)]
	for i, c in enumerate(cipher):
		buffers[i % size].append(c)
	return [''.join(b) for b in buffers]

def decrypt(cipher, key):
	
	res = []
	key = [ord(c) for c in key]
	for i, c in enumerate(cipher):
		res.append(chr(ord(c) ^ key[i % len(key)]))
	
	return ''.join(res)

def attack(cipher):
	
	size = findsize(cipher)
	res = []
	for buf in transpose(cipher, size):
		res.append(s1q3break.attack(buf))
	
	key = ''.join(chr(key) for (score, key, plain) in res)
	return key, decrypt(cipher, key)

if __name__ == '__main__':
	
	lines = []
	with open(CIPHER_FILE) as f:
		for ln in f:
			lines.append(ln.strip())
	
	print '\n\n'.join(attack(base64.b64decode(''.join(lines))))
