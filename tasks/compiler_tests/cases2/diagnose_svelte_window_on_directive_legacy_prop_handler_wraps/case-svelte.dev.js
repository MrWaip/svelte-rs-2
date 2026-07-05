import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let closeModal = $.prop($$props, "closeModal", 8);
	var $$exports = { ...$.legacy_api() };
	$.event("close", $.window, $.preventDefault(function(...$$args) {
		$.apply(closeModal, this, $$args, App, [5, 40]);
	}));
	return $.pop($$exports);
}
