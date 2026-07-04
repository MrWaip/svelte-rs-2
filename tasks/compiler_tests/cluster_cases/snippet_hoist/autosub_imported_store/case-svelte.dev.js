App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { count } from "./store_mod.js";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $count = () => ($.validate_store(count, "count"), $.store_get(count, "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const foo = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		$.next();
		var text = $.text();
		$.template_effect(() => $.set_text(text, $count()));
		$.append($$anchor, text);
	});
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => foo($$anchor), "render", App, 9, 0);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
