import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const promise = fetch("/api");
	$.await($$renderer, promise, () => {}, ({ name, age }) => {
		$$renderer.push(`<p>${$.escape(name)} is ${$.escape(age)}</p>`);
	});
	$$renderer.push(`<!--]-->`);
}
