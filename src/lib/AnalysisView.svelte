<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import LogViewTable from './util/LogViewTable.svelte';
	import { onMount } from 'svelte';
	import { open } from '@tauri-apps/api/dialog';
	import { goto } from '$app/navigation';
	import PhysicalViewTable from './util/PhysicalViewTable.svelte';

	let log_view_data: any;
	let physical_view_data: any;
	let file_name: string;
	let recent_files: { path: string; name: string }[] = [];
	let viewing_logical: boolean = true;

	async function getDisplayData() {
		const displayData_res: string = await invoke('get_logical_view');
		const displayData_res_physical: string = await invoke('get_physical_view');
		log_view_data = JSON.parse(displayData_res);
		physical_view_data = JSON.parse(displayData_res_physical);
		console.log(log_view_data);
		console.log(physical_view_data);
	}

	async function getFileData() {
		const file_name_res: string = await invoke('get_file_name');
		file_name = file_name_res;
	}

	async function getRecentlyOpened() {
		const recent_files_res: string[] = await invoke('get_recent_open');
		recent_files = recent_files_res.map((file) => {
			return JSON.parse(file);
		});
		recent_files = recent_files.slice(0, 5);
		// console.log(recent_files_res);
	}

	async function handleCompareWithClick(event: any) {
		let path = event.target.value;
		if (path === 'browse') {
			try {
				const selected_file = await open({
					multiple: false,
					filters: [{ name: 'All Files', extensions: ['doc'] }]
				});

				if (selected_file) {
					if (typeof selected_file === 'string') {
						path = selected_file;
					}
				} else {
					alert('No file selected');
				}
			} catch (error) {
				alert(error);
			}
		}

		// Here decode and save the view before going to the next page
		await invoke('read_comparison_file', { filePath: path });
		goto(`compare?compare=${viewing_logical ? 'logical' : 'physical'}`);
	}

	onMount(() => {
		getFileData();
		getDisplayData();
		getRecentlyOpened();
	});
</script>

<div class="d-flex flex-column align-items-center pt-3 px-5">
	<div class="d-flex w-50 mb-3 justify-content-between">
		<button
			class="m-0 px-2"
			class:selected-tab={viewing_logical}
			class:non-selected-tab={!viewing_logical}
			on:click={() => (viewing_logical = true)}
		>
			<h4 class="m-0">Logical</h4>
		</button>
		<button
			class="m-0 px-2"
			class:selected-tab={!viewing_logical}
			class:non-selected-tab={viewing_logical}
			on:click={() => (viewing_logical = false)}
		>
			<h4 class="m-0">Physical</h4>
		</button>
	</div>
	<div class="position-fixed" style="right: 1rem; top: 1rem;">
		<select class="form-select btn btn-primary">
			<option value="" disabled selected>Select an option</option>
			<option value="option1">Option 1</option>
			<option value="option2">Option 2</option>
			<option value="custom">Custom Action</option>
		</select>
	</div>
	<div class="" style="position: fixed; right: 1rem; top: 1rem">
		<select name="" on:change={handleCompareWithClick} class="form-select btn btn-primary">
			<option value="" disabled Selected>Compare With</option>
			{#each recent_files as file}
				<option value={file.path}>{file.name}</option>
			{/each}
			<option value="browse">Browse</option>
		</select>
	</div>
	<h4>Viewing {file_name}</h4>

	{#if viewing_logical}
		<LogViewTable structures={log_view_data} />
	{:else}
		<PhysicalViewTable data={physical_view_data} />
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
