App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.event("click", $.document.body, function(...$$args) {
		$.apply(() => $$props.onClick, this, $$args, App, [5, 22]);
	});
	$.event("custom-event", $.document.body, function(...$$args) {
		$.apply(() => $$props.onCustom, this, $$args, App, [5, 47]);
	});
	return $.pop($$exports);
}
