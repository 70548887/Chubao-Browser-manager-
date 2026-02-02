/**
 * @description RPA Bot - English
 */
export default {
  title: 'RPA Bot',

  toolbar: {
    createFlow: 'New Flow',
    importFlow: 'Import Flow',
    filterByStatus: 'Filter by Status'
  },

  status: {
    draft: 'Draft',
    ready: 'Ready',
    running: 'Running',
    paused: 'Paused',
    completed: 'Completed',
    failed: 'Failed',
    scheduled: 'Scheduled'
  },

  column: {
    name: 'Name',
    description: 'Description',
    status: 'Status',
    lastRun: 'Last Run',
    nextRun: 'Next Run',
    runCount: 'Run Count',
    successRate: 'Success Rate',
    createTime: 'Created',
    operation: 'Actions'
  },

  action: {
    run: 'Run',
    pause: 'Pause',
    stop: 'Stop',
    edit: 'Edit',
    clone: 'Clone',
    delete: 'Delete',
    viewLogs: 'View Logs',
    schedule: 'Schedule'
  },

  editor: {
    title: 'Flow Editor',
    save: 'Save',
    run: 'Run',
    undo: 'Undo',
    redo: 'Redo',
    zoomIn: 'Zoom In',
    zoomOut: 'Zoom Out',
    fitView: 'Fit View',
    nodeLibrary: 'Nodes',
    properties: 'Properties'
  },

  node: {
    browser: 'Browser',
    navigation: 'Navigation',
    click: 'Click',
    input: 'Input',
    extract: 'Extract',
    condition: 'Condition',
    loop: 'Loop',
    delay: 'Delay',
    screenshot: 'Screenshot',
    script: 'Script'
  },

  empty: {
    title: 'No Flows',
    description: 'Create your first automation flow',
    createButton: 'New Flow'
  },

  message: {
    createSuccess: 'Flow created',
    saveSuccess: 'Flow saved',
    runSuccess: 'Flow started',
    stopSuccess: 'Flow stopped',
    deleteConfirm: 'Are you sure you want to delete flow "{name}"?'
  }
}
