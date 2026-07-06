App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { timerStore } from "./store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let timerId = void 0;
		function clear() {
			if (timerId) timerStore.clearTimer(timerId);
		}
		function start() {
			timerId = timerStore.createTimer(60);
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 15, 0);
		$$renderer.push(`start</button>`);
		$.pop_element();
		$$renderer.push(` <button>`);
		$.push_element($$renderer, "button", 16, 0);
		$$renderer.push(`clear</button>`);
		$.pop_element();
		$$renderer.push(` `);
		if (timerId && $.store_get($$store_subs ??= {}, "$timerStore", timerStore)[timerId]) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`active`);
		} else {
			$$renderer.push("<!--[-1-->");
		}
		$$renderer.push(`<!--]-->`);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
