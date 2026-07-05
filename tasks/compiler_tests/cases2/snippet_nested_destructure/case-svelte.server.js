import * as $ from "svelte/internal/server";
function view($$renderer, { nested: { name = "fallback" }, list: [[first, ...rest]], ...tail }) {
	$$renderer.push(`<p>${$.escape(name)} ${$.escape(first)} ${$.escape(rest.length)} ${$.escape(tail.meta.note)}</p>`);
}
export default function App($$renderer) {
	let data = {
		nested: { name: "world" },
		list: [[
			10,
			20,
			30
		]],
		meta: { note: "ok" }
	};
	view($$renderer, data);
}
