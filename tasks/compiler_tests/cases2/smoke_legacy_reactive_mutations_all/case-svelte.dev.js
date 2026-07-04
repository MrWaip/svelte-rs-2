import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(` <button>run</button>`, 1), App[$.FILENAME], [[253, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	const $store = () => ($.validate_store(store, "store"), $.store_get(store, "$store", $$stores));
	const $objStore = () => ($.validate_store(objStore, "objStore"), $.store_get(objStore, "$objStore", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let count = $.prop($$props, "count", 12, 0);
	let obj = $.prop($$props, "obj", 28, () => ({ x: 0 }));
	let local = $.tag($.mutable_source(0), "local");
	let localObj = $.tag($.mutable_source({ x: 0 }), "localObj");
	let deep = $.tag($.mutable_source({ a: { b: { c: { x: 0 } } } }), "deep");
	let key = "x";
	let nestedKey = "b";
	const store = writable(0);
	const objStore = writable({ x: 0 });
	function script_ops() {
		count(1);
		count(count() + 2);
		count(count() - 3);
		$.update_prop(count);
		$.update_prop(count, -1);
		$.update_pre_prop(count);
		$.update_pre_prop(count, -1);
		count(count() && 4);
		count(count() || 5);
		count(count() ?? 6);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x = 7, true), 29, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x += 8, true), 30, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x++, true), 31, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x--, true), 32, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(++obj().x, true), 33, 4);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(--obj().x, true), 34, 4);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x &&= 9, true), 35, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x ||= 10, true), 36, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x ??= 11, true), 37, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj()["x"] = 7, true), 38, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj()["x"] += 8, true), 39, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(obj()["x"]++, true), 40, 2);
		$$ownership_validator.mutation(null, ["obj", "x"], obj(++obj()["x"], true), 41, 4);
		$$ownership_validator.mutation(null, ["obj", key], obj(obj()[key] = 7, true), 42, 2);
		$$ownership_validator.mutation(null, ["obj", key], obj(obj()[key] += 8, true), 43, 2);
		$$ownership_validator.mutation(null, ["obj", key], obj(obj()[key]++, true), 44, 2);
		$$ownership_validator.mutation(null, ["obj", key], obj(++obj()[key], true), 45, 4);
		$.set(local, 12);
		$.set(local, $.get(local) + 13);
		$.set(local, $.get(local) - 14);
		$.update(local);
		$.update(local, -1);
		$.update_pre(local);
		$.update_pre(local, -1);
		$.set(local, $.get(local) && 15);
		$.set(local, $.get(local) || 16);
		$.set(local, $.get(local) ?? 17);
		$.mutate(localObj, $.get(localObj).x = 18);
		$.mutate(localObj, $.get(localObj).x += 19);
		$.mutate(localObj, $.get(localObj).x++);
		$.mutate(localObj, $.get(localObj).x--);
		$.mutate(localObj, ++$.get(localObj).x);
		$.mutate(localObj, --$.get(localObj).x);
		$.mutate(localObj, $.get(localObj).x &&= 20);
		$.mutate(localObj, $.get(localObj).x ||= 21);
		$.mutate(localObj, $.get(localObj).x ??= 22);
		$.mutate(localObj, $.get(localObj)["x"] = 18);
		$.mutate(localObj, $.get(localObj)["x"] += 19);
		$.mutate(localObj, $.get(localObj)["x"]++);
		$.mutate(localObj, ++$.get(localObj)["x"]);
		$.mutate(localObj, $.get(localObj)[key] = 18);
		$.mutate(localObj, $.get(localObj)[key] += 19);
		$.mutate(localObj, $.get(localObj)[key]++);
		$.mutate(localObj, ++$.get(localObj)[key]);
		$.mutate(deep, $.get(deep).a.b.c.x = 1);
		$.mutate(deep, $.get(deep).a.b.c.x += 2);
		$.mutate(deep, $.get(deep).a.b.c.x -= 3);
		$.mutate(deep, $.get(deep).a.b.c.x++);
		$.mutate(deep, $.get(deep).a.b.c.x--);
		$.mutate(deep, ++$.get(deep).a.b.c.x);
		$.mutate(deep, --$.get(deep).a.b.c.x);
		$.mutate(deep, $.get(deep).a.b.c.x &&= 4);
		$.mutate(deep, $.get(deep).a.b.c.x ||= 5);
		$.mutate(deep, $.get(deep).a.b.c.x ??= 6);
		$.mutate(deep, $.get(deep)["a"]["b"]["c"]["x"] = 1);
		$.mutate(deep, $.get(deep)["a"]["b"]["c"]["x"] += 2);
		$.mutate(deep, $.get(deep)["a"]["b"]["c"]["x"]++);
		$.mutate(deep, ++$.get(deep)["a"]["b"]["c"]["x"]);
		$.mutate(deep, $.get(deep)[nestedKey].c[key] = 1);
		$.mutate(deep, $.get(deep)[nestedKey].c[key] += 2);
		$.mutate(deep, $.get(deep)[nestedKey].c[key]++);
		$.mutate(deep, ++$.get(deep)[nestedKey].c[key]);
		$.mutate(deep, $.get(deep).a[nestedKey].c.x = 1);
		$.mutate(deep, $.get(deep).a[nestedKey].c.x += 2);
		$.mutate(deep, $.get(deep).a[nestedKey].c.x++);
		$.mutate(deep, $.get(deep).a.b.c[key] = 1);
		$.mutate(deep, $.get(deep).a.b.c[key]++);
		$.store_set(store, 23);
		$.store_set(store, $store() + 24);
		$.store_set(store, $store() - 25);
		$.update_store(store, $store());
		$.update_store(store, $store(), -1);
		$.update_pre_store(store, $store());
		$.update_pre_store(store, $store(), -1);
		$.store_set(store, $store() && 26);
		$.store_set(store, $store() || 27);
		$.store_set(store, $store() ?? 28);
		$.store_mutate(objStore, $.untrack($objStore).x = 29, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x += 30, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x++, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x--, $.untrack($objStore));
		$.store_mutate(objStore, ++$.untrack($objStore).x, $.untrack($objStore));
		$.store_mutate(objStore, --$.untrack($objStore).x, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x &&= 31, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x ||= 32, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore).x ??= 33, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)["x"] = 29, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)["x"] += 30, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)["x"]++, $.untrack($objStore));
		$.store_mutate(objStore, ++$.untrack($objStore)["x"], $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)[key] = 29, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)[key] += 30, $.untrack($objStore));
		$.store_mutate(objStore, $.untrack($objStore)[key]++, $.untrack($objStore));
		$.store_mutate(objStore, ++$.untrack($objStore)[key], $.untrack($objStore));
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	$.next();
	var fragment = root();
	var text = $.first_child(fragment);
	var button = $.sibling(text);
	$.template_effect(() => $.set_text(text, `${count() ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj().x)) ?? ""}
${$.get(local) ?? ""}
${($.get(localObj), $.untrack(() => $.get(localObj).x)) ?? ""}
${($.get(deep), $.untrack(() => $.get(deep).a.b.c.x)) ?? ""}
${($.get(deep), $.untrack(() => $.get(deep)["a"]?.["b"]?.["c"]?.["x"])) ?? ""}
${($.get(deep), $.untrack(() => $.get(deep)?.a?.b?.c?.x)) ?? ""}
${($.get(deep), $.untrack(() => $.get(deep)[nestedKey]?.c?.[key])) ?? ""}
${$store() ?? ""}
${($objStore(), $.untrack(() => $objStore().x)) ?? ""}

${($.deep_read_state(count()), $.untrack(() => count(1))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() + 2))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() - 3))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => $.update_prop(count))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => $.update_prop(count, -1))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => $.update_pre_prop(count))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => $.update_pre_prop(count, -1))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() && 4))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() || 5))) ?? ""}
${($.deep_read_state(count()), $.untrack(() => count(count() ?? 6))) ?? ""}

${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x = 7, true), 153, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x += 8, true), 154, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x++, true), 155, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x--, true), 156, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(++obj().x, true), 157, 3))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(--obj().x, true), 158, 3))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x &&= 9, true), 159, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x ||= 10, true), 160, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj().x ??= 11, true), 161, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj()["x"] = 7, true), 162, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj()["x"] += 8, true), 163, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(obj()["x"]++, true), 164, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", "x"], obj(++obj()["x"], true), 165, 3))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", key], obj(obj()[key] = 7, true), 166, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", key], obj(obj()[key] += 8, true), 167, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", key], obj(obj()[key]++, true), 168, 1))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => $$ownership_validator.mutation(null, ["obj", key], obj(++obj()[key], true), 169, 3))) ?? ""}

${($.get(local), $.untrack(() => $.set(local, 12))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) + 13))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) - 14))) ?? ""}
${($.get(local), $.untrack(() => $.update(local))) ?? ""}
${($.get(local), $.untrack(() => $.update(local, -1))) ?? ""}
${($.get(local), $.untrack(() => $.update_pre(local))) ?? ""}
${($.get(local), $.untrack(() => $.update_pre(local, -1))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) && 15))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) || 16))) ?? ""}
${($.get(local), $.untrack(() => $.set(local, $.get(local) ?? 17))) ?? ""}

${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x = 18))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x += 19))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x++))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x--))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, ++$.get(localObj).x))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, --$.get(localObj).x))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x &&= 20))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x ||= 21))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj).x ??= 22))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)["x"] = 18))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)["x"] += 19))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)["x"]++))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, ++$.get(localObj)["x"]))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)[key] = 18))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)[key] += 19))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, $.get(localObj)[key]++))) ?? ""}
${($.get(localObj), $.untrack(() => $.mutate(localObj, ++$.get(localObj)[key]))) ?? ""}

${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a.b.c.x = 1))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a.b.c.x += 2))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a.b.c.x -= 3))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a.b.c.x++))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a.b.c.x--))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, ++$.get(deep).a.b.c.x))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, --$.get(deep).a.b.c.x))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a.b.c.x &&= 4))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a.b.c.x ||= 5))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a.b.c.x ??= 6))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep)["a"]["b"]["c"]["x"] = 1))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep)["a"]["b"]["c"]["x"] += 2))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep)["a"]["b"]["c"]["x"]++))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, ++$.get(deep)["a"]["b"]["c"]["x"]))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep)[nestedKey].c[key] = 1))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep)[nestedKey].c[key] += 2))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep)[nestedKey].c[key]++))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, ++$.get(deep)[nestedKey].c[key]))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a[nestedKey].c.x = 1))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a[nestedKey].c.x += 2))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a[nestedKey].c.x++))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a.b.c[key] = 1))) ?? ""}
${($.get(deep), $.untrack(() => $.mutate(deep, $.get(deep).a.b.c[key]++))) ?? ""}

${($store(), $.untrack(() => $.store_set(store, 23))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() + 24))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() - 25))) ?? ""}
${($store(), $.untrack(() => $.update_store(store, $store()))) ?? ""}
${($store(), $.untrack(() => $.update_store(store, $store(), -1))) ?? ""}
${($store(), $.untrack(() => $.update_pre_store(store, $store()))) ?? ""}
${($store(), $.untrack(() => $.update_pre_store(store, $store(), -1))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() && 26))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() || 27))) ?? ""}
${($store(), $.untrack(() => $.store_set(store, $store() ?? 28))) ?? ""}

${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x = 29, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x += 30, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x++, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x--, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, ++$.untrack($objStore).x, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, --$.untrack($objStore).x, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x &&= 31, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x ||= 32, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore).x ??= 33, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)["x"] = 29, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)["x"] += 30, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)["x"]++, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, ++$.untrack($objStore)["x"], $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)[key] = 29, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)[key] += 30, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, $.untrack($objStore)[key]++, $.untrack($objStore)))) ?? ""}
${($objStore(), $.untrack(() => $.store_mutate(objStore, ++$.untrack($objStore)[key], $.untrack($objStore)))) ?? ""} `));
	$.delegated("click", button, script_ops);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
$.delegate(["click"]);
