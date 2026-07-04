App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let props = $.rest_props($$props, rest_excludes, "props");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Button($$anchor, $.spread_props(() => props)), "component", App, 5, 0, { componentTag: "Button" });
	return $.pop($$exports);
}
