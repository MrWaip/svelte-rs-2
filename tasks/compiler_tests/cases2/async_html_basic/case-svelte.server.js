import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function loadContent() {
		return "<b>hello</b>";
	}
	$$renderer.child_block(async ($$renderer) => {
		$$renderer.push($.html((await $.save(loadContent()))()));
	});
}
