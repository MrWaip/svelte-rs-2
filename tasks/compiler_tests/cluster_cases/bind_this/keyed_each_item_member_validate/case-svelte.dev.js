App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let anchorRefs = {};
	let groups = $.tag_proxy($.proxy([]), "groups");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => groups, (group) => group.key, ($$anchor, group) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.validate_binding("bind:this={anchorRefs[group.key]}", [], () => anchorRefs, () => $.get(group).key, 7, 6);
		$.bind_this(div, ($$value, group) => anchorRefs[group.key] = $$value, (group) => anchorRefs?.[group.key], () => [$.get(group)]);
		$.template_effect(() => $.set_text(text, $.get(group).key));
		$.append($$anchor, div);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
