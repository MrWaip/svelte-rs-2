App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { timerStore } from "./store";
var root = $.add_locations($.from_html(`<button>start</button> <button>clear</button> <!>`, 1), App[$.FILENAME], [[15, 0], [16, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $timerStore = () => ($.validate_store(timerStore, "timerStore"), $.store_get(timerStore, "$timerStore", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let timerId = $.tag($.state(void 0), "timerId");
	function clear() {
		if ($.get(timerId)) timerStore.clearTimer($.get(timerId));
	}
	function start() {
		$.set(timerId, timerStore.createTimer(60), true);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var node = $.sibling(button_1, 2);
	{
		var consequent = ($$anchor) => {
			var text = $.text("active");
			$.append($$anchor, text);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(timerId) && $timerStore()[$.get(timerId)]) $$render(consequent);
		}), "if", App, 17, 0);
	}
	$.delegated("click", button, start);
	$.delegated("click", button_1, clear);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
