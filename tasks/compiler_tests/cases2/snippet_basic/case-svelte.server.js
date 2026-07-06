import * as $ from "svelte/internal/server";
function greeting($$renderer, name) {
	$$renderer.push(`<p>Hello ${$.escape(name)}</p>`);
}
export default function App($$renderer, $$props) {
	let { title = "world" } = $$props;
	let message = "hello";
	greeting($$renderer, message);
	$$renderer.push(`<!----> `);
	greeting($$renderer, title);
	$$renderer.push(`<!---->`);
}
