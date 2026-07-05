App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let inc;
	$.effect_root(() => {
		let count = $.tag($.state(0), "count");
		let double = $.tag($.derived(() => $.get(count) * 2), "double");
		inc = () => {
			$.update(count);
			console.log($.get(double));
		};
	});
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
