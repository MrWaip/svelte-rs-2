import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(` <button>run</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $store = () => $.store_get(store, "$store", $$stores);
	const $objStore = () => $.store_get(objStore, "$objStore", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let count = $.prop($$props, "count", 12, 0);
	let obj = $.prop($$props, "obj", 28, () => ({ x: 0 }));
	let local = $.mutable_source(0);
	let localObj = $.mutable_source({ x: 0 });
	let deep = $.mutable_source({ a: { b: { c: { x: 0 } } } });
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
		obj(obj().x = 7, true);
		obj(obj().x += 8, true);
		obj(obj().x++, true);
		obj(obj().x--, true);
		obj(++obj().x, true);
		obj(--obj().x, true);
		obj(obj().x &&= 9, true);
		obj(obj().x ||= 10, true);
		obj(obj().x ??= 11, true);
		obj(obj()["x"] = 7, true);
		obj(obj()["x"] += 8, true);
		obj(obj()["x"]++, true);
		obj(++obj()["x"], true);
		obj(obj()[key] = 7, true);
		obj(obj()[key] += 8, true);
		obj(obj()[key]++, true);
		obj(++obj()[key], true);
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

${($.deep_read_state(obj()), $.untrack(() => obj(obj().x = 7, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj().x += 8, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj().x++, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj().x--, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(++obj().x, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(--obj().x, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj().x &&= 9, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj().x ||= 10, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj().x ??= 11, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj()["x"] = 7, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj()["x"] += 8, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj()["x"]++, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(++obj()["x"], true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj()[key] = 7, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj()[key] += 8, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(obj()[key]++, true))) ?? ""}
${($.deep_read_state(obj()), $.untrack(() => obj(++obj()[key], true))) ?? ""}

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
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
