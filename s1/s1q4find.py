import s1q3break, base64

FILE = 's1q4-strings.txt'

def main():
	
	best = None
	for ln in open(FILE):
		cipher = base64.b16decode(ln.strip(), True)
		score, key, plain = s1q3break.attack(cipher)
		print score, repr(plain)
		if best is None or score > best[0]:
			best = score, key, plain
	
	return best

if __name__ == '__main__':
	print main()
