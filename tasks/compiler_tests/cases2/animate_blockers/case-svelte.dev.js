import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { flip } from "svelte/animate";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[10, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = [{
		id: 1,
		name: "a"
	}];
	var data, params;
	var $$promises = $.run([async () => data = (await $.track_reactivity_loss(fetch("/api")))(), () => params = $.tag_proxy($.proxy(data.params), "params")]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 25, () => items, (item) => item.id, ($$anchor, item) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(item).name));
		$.run_after_blockers([$$promises[1]], () => {
			$.animation(div, () => flip, () => params);
		});
		$.append($$anchor, div);
	}), "each", App, 9, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
