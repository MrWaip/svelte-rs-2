App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	function handler() {
		$.update(count);
	}
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Button($$anchor, {
		label: "Click me",
		onclick: handler,
		get count() {
			return $.get(count);
		}
	}), "component", App, 7, 0, { componentTag: "Button" });
	return $.pop($$exports);
}
