/**
 * @description Dashboard/Profile list - English
 */
export default {
  title: 'Profiles',

  toolbar: {
    createWindow: 'New Profile',
    batchOpen: 'Batch Open',
    batchClose: 'Batch Close',
    batchDelete: 'Batch Delete',
    importProfile: 'Import',
    exportProfile: 'Export',
    sortBy: 'Sort',
    filterBy: 'Filter',
    viewMode: 'View',
    listView: 'List',
    gridView: 'Grid'
  },

  sort: {
    lastUpdated: 'Last Updated',
    name: 'Name',
    createTime: 'Created',
    lastOpened: 'Last Opened'
  },

  filter: {
    all: 'All',
    running: 'Running',
    stopped: 'Stopped',
    byGroup: 'By Group',
    byTag: 'By Tag',
    byProxy: 'By Proxy'
  },

  card: {
    id: 'ID',
    group: 'Group',
    proxy: 'Proxy',
    tags: 'Tags',
    lastOpened: 'Last Opened',
    createTime: 'Created',
    noProxy: 'Direct',
    noGroup: 'Ungrouped',
    noTags: 'No Tags'
  },

  action: {
    open: 'Open',
    close: 'Close',
    edit: 'Edit',
    clone: 'Clone',
    delete: 'Delete',
    moveToGroup: 'Move to Group',
    addTags: 'Add Tags',
    setProxy: 'Set Proxy',
    viewDetail: 'Details',
    copyId: 'Copy ID'
  },

  createWizard: {
    title: 'New Profile',
    step1: 'Basic',
    step2: 'Proxy',
    step3: 'Fingerprint',
    step4: 'Preferences',
    basicInfo: 'Basic Info',
    windowName: 'Profile Name',
    windowNamePlaceholder: 'Enter profile name',
    selectGroup: 'Select Group',
    selectTags: 'Select Tags',
    remark: 'Remark',
    remarkPlaceholder: 'Enter remark (optional)'
  },

  proxySettings: {
    title: 'Proxy Settings',
    proxyType: 'Proxy Type',
    directConnect: 'Direct (No Proxy)',
    selectProxy: 'Select Existing Proxy',
    createProxy: 'Create New Proxy',
    noProxy: 'No proxy available'
  },

  fingerprintSettings: {
    title: 'Fingerprint Settings',
    autoGenerate: 'Auto Generate',
    manualConfig: 'Manual Config',
    browserCore: 'Browser Core',
    userAgent: 'User Agent',
    screenResolution: 'Screen Resolution',
    language: 'Language',
    timezone: 'Timezone',
    webGL: 'WebGL',
    canvas: 'Canvas',
    audio: 'Audio Fingerprint',
    fonts: 'Fonts',
    plugins: 'Plugins'
  },

  preferenceSettings: {
    title: 'Preferences',
    startUrl: 'Start URL',
    startUrlPlaceholder: 'Enter URL to open on start',
    downloadPath: 'Download Path',
    cookieSync: 'Cookie Sync',
    localStorage: 'LocalStorage',
    cache: 'Cache Policy'
  },

  empty: {
    title: 'No Profiles',
    description: 'Click the button above to create your first browser profile',
    createButton: 'New Profile'
  },

  batch: {
    selected: '{count} profiles selected',
    confirmDelete: 'Are you sure you want to delete {count} profiles? This cannot be undone!',
    confirmClose: 'Are you sure you want to close {count} running profiles?',
    openSuccess: 'Successfully opened {count} profiles',
    closeSuccess: 'Successfully closed {count} profiles',
    deleteSuccess: 'Successfully deleted {count} profiles'
  }
}
