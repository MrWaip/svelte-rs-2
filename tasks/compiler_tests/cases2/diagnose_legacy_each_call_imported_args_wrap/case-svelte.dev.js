import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { pick } from "./pick";
import { count } from "./count";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[9, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $count = () => ($.validate_store(count, "count"), $.store_get(count, "$count", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let kind = $.prop($$props, "kind", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => ($.deep_read_state(pick), $.deep_read_state(kind()), $count(), $.untrack(() => pick(kind(), $count()))), $.index, ($$anchor, item) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, div);
	}), "each", App, 8, 0);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
