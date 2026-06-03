let { a, b } = $state({ a: 1, b: 2 });
let [c, d] = $state([3, 4]);

export function bump() {
	a++;
	c++;
}

export function read() {
	return a + b + c + d;
}
