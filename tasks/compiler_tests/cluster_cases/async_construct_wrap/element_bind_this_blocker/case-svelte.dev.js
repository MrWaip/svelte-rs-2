import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var ref;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => ref = $.tag($.state(null), "ref")]);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.run_after_blockers([$$promises[1]], () => {
		$.bind_this(div, ($$value) => $.set(ref, $$value), () => $.get(ref));
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
