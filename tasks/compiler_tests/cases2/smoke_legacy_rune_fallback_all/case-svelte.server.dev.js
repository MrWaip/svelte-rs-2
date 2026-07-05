App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let stateBase = 0;
		let derivedBase = 1;
		let propsBase = {};
		let effectFn = () => {};
		let inspectVal = "x";
		let bindableDefault = false;
		let s = $.store_get($$store_subs ??= {}, "$state", state)(stateBase);
		let r = $.store_get($$store_subs ??= {}, "$state", state).raw(propsBase);
		let snap = $.store_get($$store_subs ??= {}, "$state", state).snapshot(propsBase);
		let d = $.store_get($$store_subs ??= {}, "$derived", derived)(derivedBase);
		let db = $.store_get($$store_subs ??= {}, "$derived", derived).by(() => derivedBase);
		let p = $.store_get($$store_subs ??= {}, "$props", props)();
		let pid = $.store_get($$store_subs ??= {}, "$props", props).id();
		let t = $.store_get($$store_subs ??= {}, "$effect", effect).tracking();
		$.store_get($$store_subs ??= {}, "$effect", effect)(effectFn);
		$.store_get($$store_subs ??= {}, "$effect", effect).pre(effectFn);
		$.store_get($$store_subs ??= {}, "$inspect", inspect)(inspectVal);
		let b = $.store_get($$store_subs ??= {}, "$bindable", bindable)(bindableDefault);
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
