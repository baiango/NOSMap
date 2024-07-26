@[export: "is_prime"]
fn is_prime(x int) bool {
	for i in 2..x / 2 + 1 {
		if x % i == 0 {
			return false
		}
	}
	return true
}

for i in 2..100 {
	if is_prime(i) {
		println('${i}')
	}
}
