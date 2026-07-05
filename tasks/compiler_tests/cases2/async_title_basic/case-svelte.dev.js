import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let title = "/api";
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		$.deferred_template_effect(($0) => {
			$.document.title = $0 ?? "";
		}, void 0, [async () => (await $.track_reactivity_loss(fetch(title)))()]);
	});
	return $.pop($$exports);
}
