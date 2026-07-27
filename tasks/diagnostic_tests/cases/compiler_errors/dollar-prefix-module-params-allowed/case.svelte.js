export function outer($$props, ...$$rest) {
	$$loop: for (let i = 0; i < 1; i++) {
		break $$loop;
	}
	return [$$props, $$rest];
}
