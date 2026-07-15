import * as $ from "svelte/internal/client";
var root = $.from_html(`<div> </div>`);
export default function App($$anchor) {
	let anchorRefs = {};
	let groups = $.proxy([]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => groups, (group) => group.key, ($$anchor, group) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.bind_this(div, ($$value, group) => anchorRefs[group.key] = $$value, (group) => anchorRefs?.[group.key], () => [$.get(group)]);
		$.template_effect(() => $.set_text(text, $.get(group).key));
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
