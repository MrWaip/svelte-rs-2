App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Tooltip from "./Tooltip.svelte";
import Icon from "./Icon.svelte";
var root = $.add_locations($.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Tooltip($$anchor, { $$slots: { activator: ($$anchor, $$slotProps) => {
		var fragment_1 = root();
		var node = $.first_child(fragment_1);
		{
			$.css_props(node, () => ({ "--color": "red" }));
			Icon(node.lastChild, { slot: "activator" });
			$.reset(node);
		}
		$.append($$anchor, fragment_1);
	} } }), "component", App, 6, 0, { componentTag: "Tooltip" });
	return $.pop($$exports);
}
