import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let onClick = $.prop($$props, "onClick", 8);
	var $$exports = { ...$.legacy_api() };
	$.event("click", $.document.body, $.preventDefault(function(...$$args) {
		$.apply(onClick, this, $$args, App, [5, 38]);
	}));
	return $.pop($$exports);
}
