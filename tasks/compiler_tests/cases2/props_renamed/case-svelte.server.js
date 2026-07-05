import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { foo: localFoo = "default", bar: localBar } = $$props;
	$$renderer.push(`<p>${$.escape(localFoo)} ${$.escape(localBar)}</p>`);
}
