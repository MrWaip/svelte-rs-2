export function outer() {
	function inner() {
		let $ = 1;
		return $;
	}
	return inner;
}
