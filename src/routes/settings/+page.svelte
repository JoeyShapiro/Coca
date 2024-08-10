<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
  
    import 'bootstrap/dist/css/bootstrap.min.css';
    import 'bootstrap/dist/js/bootstrap.bundle.min.js';
    import { onMount } from 'svelte';

    import Header from '../../components/Header.svelte';
    import Sidebar from '../../components/Sidebar.svelte';
  
    class UserSettings {
      precision: number;

      constructor(precision: number) {
        this.precision = precision;
      }
    }

    let settings = new UserSettings(0);
  
    async function getSettings() {
      settings = await invoke("get_settings");
    }

    async function setSettings() {
      await invoke("set_settings", { userSettings: settings });
    }
  
    onMount(() => {
      getSettings();
    });
  
  </script>
  
  <div class="">
    <Header />
    <div class="container-fluid body">
      <div class="row">
          <Sidebar app="{undefined}" />
  
      <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
        <div class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom">
          <h1 class="h2">Settings</h1>
        </div>
        <div class="row align-items-center">
          <div class="col-auto">
            <label for="precision" class="form-label me-2">Precision</label>
          </div>
          <div class="col-auto">
            <input type="number" min="0" max="1" step="0.001" bind:value={settings.precision} class="form-control">
          </div>
          <div class="col-auto">
            <input type="range" min="0" max="1" step="0.001" bind:value={settings.precision} class="form-range">
          </div>
        </div>
        <!-- save -->
        <div class="row align-items-center mt-3 position-absolute bottom-0 end-0 p-3">
          <div class="col-auto">
            <button class="btn btn-primary" on:click={setSettings}>Save</button>
          </div>
      </main>
    </div>
  </div>
  
  </div>
  
  <style>
  
  .body {
    padding-top: 50px; /* Adjust this value based on your navbar height */
  }
  
  main {
      margin-left: 250px; /* Should match the sidebar width */
  }
  </style>
  