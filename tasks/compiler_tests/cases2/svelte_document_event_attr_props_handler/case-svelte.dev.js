App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.event("click", $.document, function(...$$args) {
		$.apply(() => $$props.onClick, this, $$args, App, [5, 26]);
	});
	$.event("custom-event", $.document, function(...$$args) {
		$.apply(() => $$props.onCustom, this, $$args, App, [5, 51]);
	});
	return $.pop($$exports);
}
