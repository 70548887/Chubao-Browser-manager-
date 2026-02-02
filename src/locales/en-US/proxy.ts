/**
 * @description Proxy management - English
 */
export default {
  title: 'Proxy Management',

  toolbar: {
    addProxy: 'Add Proxy',
    batchImport: 'Batch Import',
    batchExport: 'Batch Export',
    batchDelete: 'Batch Delete',
    batchCheck: 'Batch Check',
    filterByType: 'Filter by Type',
    filterByStatus: 'Filter by Status'
  },

  type: {
    http: 'HTTP',
    https: 'HTTPS',
    socks4: 'SOCKS4',
    socks5: 'SOCKS5',
    ssh: 'SSH'
  },

  status: {
    valid: 'Valid',
    invalid: 'Invalid',
    checking: 'Checking',
    unchecked: 'Unchecked',
    expired: 'Expired'
  },

  column: {
    name: 'Name',
    type: 'Type',
    host: 'Host',
    port: 'Port',
    username: 'Username',
    password: 'Password',
    status: 'Status',
    delay: 'Latency',
    location: 'Location',
    usedBy: 'Used By',
    expireTime: 'Expires',
    lastCheck: 'Last Check',
    createTime: 'Created',
    operation: 'Actions'
  },

  form: {
    title: 'Proxy Info',
    addTitle: 'Add Proxy',
    editTitle: 'Edit Proxy',
    name: 'Name',
    namePlaceholder: 'Enter proxy name',
    type: 'Type',
    host: 'Host',
    hostPlaceholder: 'Enter host or IP',
    port: 'Port',
    portPlaceholder: 'Enter port',
    username: 'Username',
    usernamePlaceholder: 'Enter username (optional)',
    password: 'Password',
    passwordPlaceholder: 'Enter password (optional)',
    remark: 'Remark',
    remarkPlaceholder: 'Enter remark'
  },

  import: {
    title: 'Batch Import Proxies',
    description: 'Supported formats:',
    format1: 'host:port',
    format2: 'host:port:username:password',
    format3: 'type://host:port',
    format4: 'type://username:password@host:port',
    placeholder: 'Enter proxy info, one per line',
    parseResult: 'Parse result: {success} success, {failed} failed',
    importButton: 'Import'
  },

  action: {
    check: 'Check',
    edit: 'Edit',
    delete: 'Delete',
    copyInfo: 'Copy Info'
  },

  checkResult: {
    success: 'Connected',
    failed: 'Failed',
    timeout: 'Timeout',
    delay: 'Latency: {ms}ms',
    location: 'Location: {location}'
  },

  empty: {
    title: 'No Proxies',
    description: 'Click the button above to add your first proxy',
    addButton: 'Add Proxy'
  },

  message: {
    checkSuccess: 'Proxy check successful, latency {ms}ms',
    checkFailed: 'Proxy check failed: {reason}',
    deleteConfirm: 'Are you sure you want to delete proxy "{name}"?',
    deleteWithProfiles: 'This proxy is used by {count} profiles. They will be set to direct mode. Continue?',
    importSuccess: 'Successfully imported {count} proxies',
    exportSuccess: 'Successfully exported {count} proxies'
  }
}
