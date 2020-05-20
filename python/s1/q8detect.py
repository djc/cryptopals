import base64, collections

def find(ciphers):
	
	best = None
	for idx, cipher in enumerate(ciphers):
		
		assert len(cipher) == 160
		chunks = collections.Counter()
		for i in range(0, len(cipher), 16):
			chunks[cipher[i:i + 16]] += 1
		
		most = chunks.most_common(1)[0][1]
		if best is None or best[0] < most:
			best = most, idx, cipher
	
	return best

if __name__ == '__main__':
	
	ciphers = []
	with open('q8-ciphers.txt') as f:
		for ln in f:
			ciphers.append(base64.b16decode(ln.strip(), True))
	
	print find(ciphers)
