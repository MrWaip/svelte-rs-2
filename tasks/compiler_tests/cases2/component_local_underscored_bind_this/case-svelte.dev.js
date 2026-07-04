App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let refs = $.tag_proxy($.proxy([]), "refs");
	const Derived_1 = $.tag($.derived(() => Widget), "Derived_1");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	$.validate_binding("bind:this={refs[1]}", [], () => refs, () => 1, 10, 11);
	$.add_svelte_meta(() => $.component(node, () => $.get(Derived_1), ($$anchor, Derived_1_1) => {
		$.bind_this(Derived_1_1($$anchor, {}), ($$value) => refs[1] = $$value, () => refs?.[1]);
	}), "component", App, 10, 0, { componentTag: "Derived_1" });
	var node_1 = $.sibling(node, 2);
	{
		var consequent = ($$anchor) => {
			const Const_0 = $.tag($.derived(() => Widget), "Const_0");
			$.get(Const_0);
			var fragment_1 = $.comment();
			var node_2 = $.first_child(fragment_1);
			$.validate_binding("bind:this={refs[0]}", [], () => refs, () => 0, 14, 10);
			$.add_svelte_meta(() => $.component(node_2, () => $.get(Const_0), ($$anchor, Const_0_1) => {
				$.bind_this(Const_0_1($$anchor, {}), ($$value) => refs[0] = $$value, () => refs?.[0]);
			}), "component", App, 14, 1, { componentTag: "Const_0" });
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node_1, ($$render) => {
			if (true) $$render(consequent);
		}), "if", App, 12, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
