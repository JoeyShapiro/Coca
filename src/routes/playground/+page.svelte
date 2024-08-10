<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
  
    import 'bootstrap/dist/css/bootstrap.min.css';
    import 'bootstrap/dist/js/bootstrap.bundle.min.js';
    import { onMount } from 'svelte';

    import Header from '../../components/Header.svelte';
    import Sidebar from '../../components/Sidebar.svelte';
  
    class Application {
      name: string;
      presses: number;
      combos: number;
  
      constructor(name: string, presses: number, combos: number) {
        this.name = name;
        this.presses = presses;
        this.combos = combos;
      }
    }
  
    let apps: Application[] = [];
  
    async function applications() {
      apps = await invoke("applications");
  
      // sort by presses
      apps.sort((a, b) => b.presses - a.presses);
    }
  
    onMount(() => {
      applications();
    });
  
  </script>
  
  <div class="">
    <Header />
    <div class="container-fluid body">
      <div class="row">
          <Sidebar app={undefined} />
  
      <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
        <img src="/gamepad.svg"  alt="combo" />
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
  