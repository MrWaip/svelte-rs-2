import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Foo($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const onClick = $.derived_safe_equal(() => $$slotProps.onClick);
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(onClick)));
			$.append($$anchor, p);
		} }
	}), "component", App, 5, 0, { componentTag: "Foo" });
	return $.pop($$exports);
}
