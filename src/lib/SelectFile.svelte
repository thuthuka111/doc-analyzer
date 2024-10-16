<script lang="ts">
	import { goto } from '$app/navigation';
	import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	let recent_files: { path: string; name: string }[] = [];
	let recent_compare_files: [{ path: string; name: string }, { path: string; name: string }][] = [];

	onMount(() => {
		getRecentlyOpened();
		getRecentlyCompared();
	});

	async function browseFiles() {
		try {
			const selected_file = await open({
				multiple: false,
				filters: [{ name: 'All Files', extensions: ['doc'] }]
			});

			if (selected_file) {
				if (typeof selected_file === 'string') {
					analyseFile(selected_file);
				}
			}
		} catch (error) {
			alert(error);
			// Some sort of error occured
		}
	}

	async function compareFile(pair: [{ path: string; name: string }, { path: string; name: string }]){
		console.log(pair);
		try {
			await invoke('read_file', { filePath: pair[0].path });
			await invoke('read_comparison_file', { filePath: pair[1].path });
			goto('/compare');
		} catch (error) {
			alert("Error from rust: " + error);
		}
	}

	async function analyseFile(path: string) {
		try {
			await invoke('read_file', { filePath: path });
			goto('/analyse');
		} catch (error) {
			alert("Error from rust: " + error);
		}
	}

	async function getRecentlyCompared() {
		const recent_compare_res: string[] = await invoke('get_recent_compare');
		recent_compare_files = recent_compare_res.map((file) => {
			return JSON.parse(file);
		});
		recent_compare_files = recent_compare_files.slice(0, 5);
		// console.log(recent_compare_res);
		console.log(recent_compare_files);
	}

	async function getRecentlyOpened() {
		const recent_files_res: string[] = await invoke('get_recent_open');
		recent_files = recent_files_res.map((file) => {
			return JSON.parse(file);
		});
		recent_files = recent_files.slice(0, 5);
		// console.log(recent_files_res);
	}
</script>

<div class="d-flex w-100 justify-content-center align-items-center" style="height: 100vh">
	<div class="d-flex justify-content-center align-items-center" style="flex-basis: 63%;">
		<div>
			<p>Select File/s to analyse</p>
			<button on:click={browseFiles} class="btn btn-success">Browse</button>
		</div>
	</div>
	<div class="d-flex flex-column justify-content-between" style="flex-basis: 27%; height: 85%">
		<div style="height: 45%;">
			<p class="text-center">Recently Viewed</p>
			{#each recent_files as file}
				<button
					on:click={() => analyseFile(file.path)}
					style="color: white; width: 100%;"
					class="recent-btn p-1 mb-1"
				>
					<p class="text-center m-0">{file.name}</p>
				</button>
			{/each}
		</div>
		<div style="height: 45%;">
			<p class="text-center">Recently Compared</p>
			{#each recent_compare_files as compare_pair}
			{@const ref_file = compare_pair[0]}
			{@const comp_file = compare_pair[1]}	
				<button
					on:click={() => compareFile(compare_pair)}
					style="color: white; width: 100%;"
					class="recent-btn p-1 mb-1"
				>
					<p class="text-center m-0">{ref_file.name} vs {comp_file.name}</p>
				</button>
			{/each}
		</div>
	</div>
</div>

<style>
	.recent-btn {
		border: none;
		background-color: #e9ecef15;
		border-radius: 5px;
		/* cursor: pointer; */
	}

	.recent-btn:hover {
		background-color: #e9ecef25;
	}
</style>
