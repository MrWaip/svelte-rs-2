import * as $ from "svelte/internal/server";
function row($$renderer, label, { id }, [value]) {
	$$renderer.push(`<p>${$.escape(label)}: ${$.escape(id)} = ${$.escape(value)}</p>`);
}
export default function App($$renderer) {
	let items = [{ id: 1 }];
	row($$renderer, "test", items[0], [42]);
}
