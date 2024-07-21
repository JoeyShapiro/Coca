<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
  
    import 'bootstrap/dist/css/bootstrap.min.css';
    import 'bootstrap/dist/js/bootstrap.bundle.min.js';
    import { onMount } from 'svelte';
  
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
  <nav class="p-2 border-bottom navbar-expand-lg bg-light fixed-top">
    <div class="container">
      <div class="d-flex flex-wrap align-items-center justify-content-center justify-content-lg-start">
        <div class="dropdown text-end">
          <a href="/" class="d-block link-body-emphasis text-decoration-none dropdown-toggle" data-bs-toggle="dropdown" aria-expanded="false">
            <img src="/favicon.png" alt="mdo" width="32" height="32" class="rounded-circle">
          </a>
          <ul class="dropdown-menu text-small">
            <li><a class="dropdown-item" href="#">New project...</a></li>
            <li><a class="dropdown-item" href="#">Settings</a></li>
            <li><a class="dropdown-item" href="#">Profile</a></li>
          </ul>
        </div>
  
        <ul class="nav col-12 col-lg-auto me-lg-auto mb-2 justify-content-center mb-md-0">
          <li><a href="/" class="nav-link px-2 link-secondary">Dashboard</a></li>
          <li><a href="/applications" class="nav-link px-2 link-body-emphasis">Applications</a></li>
          <li><a href="/playground" class="nav-link px-2 link-body-emphasis">Playground</a></li>
        </ul>
  
        <form class="col-12 col-lg-auto mb-3 mb-lg-0 me-lg-3" role="search">
          <input type="search" class="form-control" placeholder="Search..." aria-label="Search">
        </form>
      </div>
    </div>
  </nav>
  
  <div class="container-fluid body">
    <div class="row">
      <div class="sidebar border border-right col-md-3 col-lg-2 p-0 bg-body-tertiary">
        <div class="offcanvas-md offcanvas-end bg-body-tertiary" tabindex="-1" id="sidebarMenu" aria-labelledby="sidebarMenuLabel">
          <div class="offcanvas-body d-md-flex flex-column p-0 pt-lg-3 overflow-y-auto">
            <ul class="nav flex-column">
              <li class="nav-item">
                <a class="nav-link d-flex align-items-center gap-2 link-secondary" href="#">
                  <img src="/overview.svg" class="bi" alt="overview" />
                  Overview
                </a>
              </li>
              <li class="nav-item">
                <a class="nav-link d-flex align-items-center gap-2 link-body-emphasis" href="#">
                  <img src="/heatmap.svg" class="bi" alt="overview" />
                  Heatmap
                </a>
              </li>
              <li class="nav-item">
                <a class="nav-link d-flex align-items-center gap-2 link-body-emphasis" href="#">
                  <img src="/combo.svg" class="bi" alt="combo" />
                  Combos
                </a>
              </li>
              <li class="nav-item">
                <a class="nav-link d-flex align-items-center gap-2 link-body-emphasis" href="#">
                  <!-- <img src="/overview.svg" class="logo" alt="overview" /> -->
                  Integrations
                </a>
              </li>
            </ul>
          </div>
        </div>
      </div>
  
      <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
        <img src="/gamepad.svg"  alt="combo" />
      </main>
    </div>
  </div>
  
  </div>
  
  <style>
    .bi {
    width: 1em;
    height: 1em;
  }
  
  .body {
    padding-top: 50px; /* Adjust this value based on your navbar height */
  }
  
  .sidebar {
    position: fixed;
    top: 56px; /* Should match the body padding-top */
    left: 0;
    height: calc(100vh - 56px);
    z-index: 1000;
    padding-top: 20px;
    background-color: #f8f9fa;
    overflow-y: auto;
  }
  
  main {
      margin-left: 250px; /* Should match the sidebar width */
  }
  
  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }
  </style>
  