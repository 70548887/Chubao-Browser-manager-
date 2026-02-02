# Chubao Fingerprint Browser - UI Design & Implementation Plan

## Goal
Design and implement the UI for the "Chubao Fingerprint Browser" launcher, utilizing a "NetEase Cross-border Browser" inspired aesthetic (Premium Professional, Dark Mode, High Density).

## Design Specification

### Visual Style ("Deep Night Premium")
- **Theme**: Dark Mode base with high contrast text.
- **Palette**:
  - Background: `#1a1b1e` (Deep Blue/Black)
  - Surface: `#25262b` (Card/Panel Background)
  - Primary: `#409EFF` (Tech Blue) - Action buttons, Active states.
  - Success: `#67C23A` (Safe Green) - Status indicators (Active/Running).
  - Warning: `#E6A23C` (Warning Orange) - Alerts.
  - Text: `#E5EAF3` (Primary), `#A3A6AD` (Secondary).
- **Typography**: System UI (`Segoe UI`, `Inter`, `PingFang SC`) for crisp rendering.
- **Components**:
  - **Glassmorphism**: Subtle use on sidebar and floating modals.
  - **Borders**: Thin, subtle borders (`#363b45`) to define areas without visual clutter.
  - **Density**: Compact spacing to show maximum data (Environment lists) without scrolling.

### Menu Structure (Sidebar Navigation)
1.  **Environments (环境管理)** [Home/Dashboard]
    - Main view. List of browser profiles with status, proxy info, and quick actions (Launch/Stop).
2.  **Fingerprints (指纹配置)**
    - Fingerprint templates, random generation settings, and kernel parameters.
3.  **Window Control (窗口群控)**
    - Layout management (Grid 3x2, 4x4), Window alignment, functionality to Bring to Front / Hide All.
4.  **Network (网络中转)**
    - Proxy Bridge monitor, Network interface selection, Global proxy settings.
5.  **Automation (自动化任务)**
    - Script manager, Scheduled tasks, RPA task status.
6.  **Settings (系统设置)**
    - Global preferences, Kernel path, Cache clearing.

### Layout
- **Sidebar (Left)**: Collapsible, 240px width. Logo at top ("触宝浏览器"), Navigation items, User profile at bottom.
- **Main Content (Right)**:
  - **Header**: Breadcrumbs, Global Search (Environment ID/Name), System Status (CPU/RAM).
  - **Workspace**: The active view content (e.g., Data Grid for Environments).

## User Review Required
> [!IMPORTANT]
> **NetEase Style Interpretation**: Since direct screenshots are unavailable, I am inferring the style as "Modern Dark Enterprise UI". Please confirm if specific "NetEase" elements (e.g., specific shade of red/brand color) are needed.

> [!NOTE]
> **Logo**: "Chubao Fingerprint Browser" (触宝指纹浏览器) text will be used.

## Proposed Changes

### Frontend Architecture (Vue 3 + Element Plus)
#### [NEW] `src/layouts/MainLayout.vue`
- Implementation of the Sidebar + Header + Content Area layout.
- Responsive design handling.

#### [NEW] `src/components/Sidebar.vue`
- Navigation menu implementation using Element Plus Menu.
- Integration with Router.

#### [NEW] `src/views/EnvironmentList.vue`
- The main dashboard.
- **DataGrid**: Virtualized table for high performance with 1000+ profiles.
- **Columns**: ID, Name, Group, Proxy (Country Flag), Status (Running/Stopped), Last Active.
- **Actions**: Batch Launch, Edit, Delete.

#### [NEW] `src/views/FingerprintConfig.vue`
- Complex form for fingerprint parameters (Canvas noise, WebGL, AudioContext).
- Visual "Fingerprint Strength" meter.

### Backend Support (Rust)
- Ensure specific Tauri commands exist for UI data fetching (mocked if backend not fully ready).

## Verification Plan

### Automated Tests
- **Component Tests**: Unit checks for Sidebar navigation state.
- **Build Test**: Ensure `npm run build` succeeds with new components.

### Manual Verification
1.  **Visual Inspection**:
    - Launch the app.
    - Verify "Dark Mode" aesthetic matches the description.
    - Check Sidebar navigation switches views correctly.
2.  **Responsiveness**: Resize window to ensure layout adapts (Sidebar collapsing).
3.  **Mock Data**: Verify Environment List displays mock data correctly with status indicators.
