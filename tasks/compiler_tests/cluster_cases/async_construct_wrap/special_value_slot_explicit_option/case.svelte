<script>await Promise.resolve(); let a = 'a';</script>
<select value={await Promise.resolve('x')}>
	<option value={await Promise.resolve('y')}>{a}</option>
</select>
