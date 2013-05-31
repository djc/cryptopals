import base64, string, collections, itertools

CIPHER = '1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736'
FREQ = {c: i for (i, c) in enumerate('etaoinshrdlcumwfgypbvkjxqz')}

def score(chars):
	
	res = sum(1 for c in chars if c in string.printable)
	letters = (c.lower() for c in chars if c in string.letters)
	freq = collections.Counter(letters)
	nums = sorted(((f, c) for (c, f) in freq.iteritems()), reverse=True)
	ranked = {c: i for (i, (f, c)) in enumerate(nums)}
	for pair in itertools.combinations(ranked, 2):
		
		if pair[0] == pair[1]:
			continue
		
		diff = cmp(ranked[pair[0]], ranked[pair[1]])
		expect = cmp(FREQ[pair[0]], FREQ[pair[1]])
		if diff == expect:
			res += 1
	
	return res

def decrypt(cipher, key):
	return ''.join(chr(ord(c) ^ key) for c in cipher)

def attack(cipher):
	
	best = None
	for test in range(256):
		candidate = decrypt(cipher, test)
		val = score(candidate)
		if best is None or val > best[0]:
			best = val, test, candidate
	
	return best

if __name__ == '__main__':
	print attack(base64.b16decode(CIPHER, True))
