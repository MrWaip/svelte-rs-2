import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let props = {
		class: "foo",
		id: "bar"
	};
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attributes({ ...props })}`);
	}, () => {
		$$renderer.push(`content`);
	});
}
