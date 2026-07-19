export function load(url) {
	return import(
		/* @vite-ignore */
		url
	);
}
