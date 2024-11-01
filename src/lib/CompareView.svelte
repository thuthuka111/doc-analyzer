<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import ComparePhysical from './util/ComparePhysical.svelte';
	import CompareLogical from './util/CompareLogical.svelte';

	let compare_physical_data: any;
	let compare_logical_data: any;
	let file_name: string;
	let compare_file_name: string;
	let recent_files: { path: string; name: string }[] = [];
	let comparing_logical: boolean = true;

	async function getDisplayData() {
		const compare_physical_data_res: string = await invoke('get_compare_physical_view');
		compare_physical_data = JSON.parse(compare_physical_data_res);
		console.log(compare_physical_data);

		const compare_logical_data_res: string = await invoke('get_compare_logical_view');
		compare_logical_data = JSON.parse(compare_logical_data_res);
		console.log(compare_logical_data);
	}

	async function getFileData() {
		const file_name_res: string = await invoke('get_file_name');
		const compare_file_name_res: string = await invoke('get_comparison_file_name');
		file_name = file_name_res;
		compare_file_name = compare_file_name_res;
	}

	async function getRecentlyOpened() {
		const recent_files_res: string[] = await invoke('get_recent_open');
		recent_files = recent_files_res.map((file) => {
			return JSON.parse(file);
		});
		recent_files = recent_files.slice(0, 5);
	}

	onMount(() => {
		const urlParams = new URLSearchParams(window.location.search);
		const compareParam = urlParams.get('compare');
		if (compareParam) {
			comparing_logical = compareParam === 'logical';
		}

		getFileData();
		getDisplayData();
		getRecentlyOpened();
	});
</script>

<div class="d-flex flex-column align-items-center pt-3 px-5">
	<div class="d-flex w-50 mb-3 justify-content-between">
		<button
			class="m-0 px-2"
			class:selected-tab={comparing_logical}
			class:non-selected-tab={!comparing_logical}
			on:click={() => (comparing_logical = true)}
		>
			<h4 class="m-0">Logical</h4>
		</button>
		<button
			class="m-0 px-2"
			class:selected-tab={!comparing_logical}
			class:non-selected-tab={comparing_logical}
			on:click={() => (comparing_logical = false)}
		>
			<h4 class="m-0">Physical</h4>
		</button>
	</div>

	<div class="" style="position: fixed; right: 1rem; top: 1rem">
		<!-- but something here idk, maybe a back button -->
	</div>
	<p>Comparing {file_name} and {compare_file_name}</p>

	{#if comparing_logical}
		<CompareLogical data={compare_logical_data} ref_file_name={file_name} {compare_file_name} />
	{:else}
		<ComparePhysical data={compare_physical_data} ref_file_name={file_name} {compare_file_name} />
	{/if}
</div>

<style>
	.selected-tab {
		all: unset;
		border-bottom: 2px solid white;
	}
	.non-selected-tab {
		all: unset;
		border-bottom: 2px solid rgb(184, 184, 184);
	}
</style>
