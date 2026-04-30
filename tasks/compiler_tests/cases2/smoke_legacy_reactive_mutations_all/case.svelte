<svelte:options runes={false} />
<script>
	import { writable } from 'svelte/store';

	export let count = 0;
	export let obj = { x: 0 };

	let local = 0;
	let localObj = { x: 0 };
	let deep = { a: { b: { c: { x: 0 } } } };
	let key = 'x';
	let nestedKey = 'b';

	const store = writable(0);
	const objStore = writable({ x: 0 });

	function script_ops() {
		count = 1;
		count += 2;
		count -= 3;
		count++;
		count--;
		++count;
		--count;
		count &&= 4;
		count ||= 5;
		count ??= 6;

		obj.x = 7;
		obj.x += 8;
		obj.x++;
		obj.x--;
		++obj.x;
		--obj.x;
		obj.x &&= 9;
		obj.x ||= 10;
		obj.x ??= 11;
		obj["x"] = 7;
		obj["x"] += 8;
		obj["x"]++;
		++obj["x"];
		obj[key] = 7;
		obj[key] += 8;
		obj[key]++;
		++obj[key];

		local = 12;
		local += 13;
		local -= 14;
		local++;
		local--;
		++local;
		--local;
		local &&= 15;
		local ||= 16;
		local ??= 17;

		localObj.x = 18;
		localObj.x += 19;
		localObj.x++;
		localObj.x--;
		++localObj.x;
		--localObj.x;
		localObj.x &&= 20;
		localObj.x ||= 21;
		localObj.x ??= 22;
		localObj["x"] = 18;
		localObj["x"] += 19;
		localObj["x"]++;
		++localObj["x"];
		localObj[key] = 18;
		localObj[key] += 19;
		localObj[key]++;
		++localObj[key];

		deep.a.b.c.x = 1;
		deep.a.b.c.x += 2;
		deep.a.b.c.x -= 3;
		deep.a.b.c.x++;
		deep.a.b.c.x--;
		++deep.a.b.c.x;
		--deep.a.b.c.x;
		deep.a.b.c.x &&= 4;
		deep.a.b.c.x ||= 5;
		deep.a.b.c.x ??= 6;
		deep["a"]["b"]["c"]["x"] = 1;
		deep["a"]["b"]["c"]["x"] += 2;
		deep["a"]["b"]["c"]["x"]++;
		++deep["a"]["b"]["c"]["x"];
		deep[nestedKey].c[key] = 1;
		deep[nestedKey].c[key] += 2;
		deep[nestedKey].c[key]++;
		++deep[nestedKey].c[key];
		deep.a[nestedKey].c.x = 1;
		deep.a[nestedKey].c.x += 2;
		deep.a[nestedKey].c.x++;
		deep.a.b.c[key] = 1;
		deep.a.b.c[key]++;

		$store = 23;
		$store += 24;
		$store -= 25;
		$store++;
		$store--;
		++$store;
		--$store;
		$store &&= 26;
		$store ||= 27;
		$store ??= 28;

		$objStore.x = 29;
		$objStore.x += 30;
		$objStore.x++;
		$objStore.x--;
		++$objStore.x;
		--$objStore.x;
		$objStore.x &&= 31;
		$objStore.x ||= 32;
		$objStore.x ??= 33;
		$objStore["x"] = 29;
		$objStore["x"] += 30;
		$objStore["x"]++;
		++$objStore["x"];
		$objStore[key] = 29;
		$objStore[key] += 30;
		$objStore[key]++;
		++$objStore[key];
	}
</script>

{count}
{obj.x}
{local}
{localObj.x}
{deep.a.b.c.x}
{deep["a"]?.["b"]?.["c"]?.["x"]}
{deep?.a?.b?.c?.x}
{deep[nestedKey]?.c?.[key]}
{$store}
{$objStore.x}

{count = 1}
{count += 2}
{count -= 3}
{count++}
{count--}
{++count}
{--count}
{count &&= 4}
{count ||= 5}
{count ??= 6}

{obj.x = 7}
{obj.x += 8}
{obj.x++}
{obj.x--}
{++obj.x}
{--obj.x}
{obj.x &&= 9}
{obj.x ||= 10}
{obj.x ??= 11}
{obj["x"] = 7}
{obj["x"] += 8}
{obj["x"]++}
{++obj["x"]}
{obj[key] = 7}
{obj[key] += 8}
{obj[key]++}
{++obj[key]}

{local = 12}
{local += 13}
{local -= 14}
{local++}
{local--}
{++local}
{--local}
{local &&= 15}
{local ||= 16}
{local ??= 17}

{localObj.x = 18}
{localObj.x += 19}
{localObj.x++}
{localObj.x--}
{++localObj.x}
{--localObj.x}
{localObj.x &&= 20}
{localObj.x ||= 21}
{localObj.x ??= 22}
{localObj["x"] = 18}
{localObj["x"] += 19}
{localObj["x"]++}
{++localObj["x"]}
{localObj[key] = 18}
{localObj[key] += 19}
{localObj[key]++}
{++localObj[key]}

{deep.a.b.c.x = 1}
{deep.a.b.c.x += 2}
{deep.a.b.c.x -= 3}
{deep.a.b.c.x++}
{deep.a.b.c.x--}
{++deep.a.b.c.x}
{--deep.a.b.c.x}
{deep.a.b.c.x &&= 4}
{deep.a.b.c.x ||= 5}
{deep.a.b.c.x ??= 6}
{deep["a"]["b"]["c"]["x"] = 1}
{deep["a"]["b"]["c"]["x"] += 2}
{deep["a"]["b"]["c"]["x"]++}
{++deep["a"]["b"]["c"]["x"]}
{deep[nestedKey].c[key] = 1}
{deep[nestedKey].c[key] += 2}
{deep[nestedKey].c[key]++}
{++deep[nestedKey].c[key]}
{deep.a[nestedKey].c.x = 1}
{deep.a[nestedKey].c.x += 2}
{deep.a[nestedKey].c.x++}
{deep.a.b.c[key] = 1}
{deep.a.b.c[key]++}

{$store = 23}
{$store += 24}
{$store -= 25}
{$store++}
{$store--}
{++$store}
{--$store}
{$store &&= 26}
{$store ||= 27}
{$store ??= 28}

{$objStore.x = 29}
{$objStore.x += 30}
{$objStore.x++}
{$objStore.x--}
{++$objStore.x}
{--$objStore.x}
{$objStore.x &&= 31}
{$objStore.x ||= 32}
{$objStore.x ??= 33}
{$objStore["x"] = 29}
{$objStore["x"] += 30}
{$objStore["x"]++}
{++$objStore["x"]}
{$objStore[key] = 29}
{$objStore[key] += 30}
{$objStore[key]++}
{++$objStore[key]}

<button onclick={script_ops}>run</button>
