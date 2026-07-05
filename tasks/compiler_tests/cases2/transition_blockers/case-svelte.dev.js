import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { fade } from "svelte/transition";
var root = $.add_locations($.from_html(`<div>hello</div>`), App[$.FILENAME], [[9, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var data, params;
	var $$promises = $.run([async () => data = (await $.track_reactivity_loss(fetch("/api")))(), () => params = $.tag_proxy($.proxy(data.params), "params")]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root();
			$.run_after_blockers([$$promises[1]], () => {
				$.transition(3, div, () => fade, () => params);
			});
			$.append($$anchor, div);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (true) $$render(consequent);
		}), "if", App, 8, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
