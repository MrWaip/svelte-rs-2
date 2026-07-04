App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let show = true;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (show) $$render(consequent);
		}), "if", App, 4, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
