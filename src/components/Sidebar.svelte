<div class="sidebar border border-right col-md-3 col-lg-2 p-0 bg-body-tertiary">
    <div class="offcanvas-md offcanvas-end bg-body-tertiary h-100" tabindex="-1" id="sidebarMenu" aria-labelledby="sidebarMenuLabel">
        <div class="offcanvas-body d-md-flex flex-column p-0 pt-lg-3 overflow-y-auto h-100">
        <ul class="nav flex-column mb-auto">
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
            
            <div class="mt-auto p-3">
                <a class="nav-link d-flex align-items-center gap-2 link-body-emphasis" href="/settings">
                    <img src="/settings.svg" class="bi" alt="settings" />
                    Settings
                </a>
                <a class="nav-link d-flex align-items-center gap-2 link-body-emphasis"
                    href="https://github.com/JoeyShapiro" target="_blank" rel="noopener noreferrer">
                    <img src="/github-mark.svg" class="bi" alt="plug" />
                    JoeyShapiro
                </a>
            </div>
        </div>
    </div>
</div>

<style>
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
    
    .bi {
        width: 1em;
        height: 1em;
    }
</style>