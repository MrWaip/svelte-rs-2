import * as $ from "svelte/internal/server";
import Component from "./Component.svelte";
export default function App($$renderer) {
	$.css_props($$renderer, true, { "--color": "red" }, () => {
		Component($$renderer, {});
	});
}
