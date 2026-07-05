import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<select>`);
	$$renderer.option({ value: "b" }, ($$renderer) => {
		$$renderer.push(`Two`);
	});
	$$renderer.push(`</select>`);
}
