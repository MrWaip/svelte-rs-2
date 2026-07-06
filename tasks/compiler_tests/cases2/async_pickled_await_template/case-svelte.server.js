import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let promise = Promise.resolve("hello");
	const suffix = "!";
	$$renderer.push(`<p>`);
	$$renderer.push(async () => $.escape((await $.save(promise))() + suffix));
	$$renderer.push(`</p>`);
}
