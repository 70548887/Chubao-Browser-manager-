/**
 * @description Recycle Bin - English
 */
export default {
  title: 'Recycle Bin',

  toolbar: {
    restore: 'Restore',
    permanentDelete: 'Delete Permanently',
    emptyRecycle: 'Empty Recycle Bin',
    filterByType: 'Filter by Type'
  },

  type: {
    all: 'All',
    profile: 'Profile',
    group: 'Group',
    proxy: 'Proxy',
    tag: 'Tag',
    flow: 'Flow'
  },

  column: {
    name: 'Name',
    type: 'Type',
    deleteTime: 'Deleted',
    expireTime: 'Expires',
    deletedBy: 'Deleted By',
    operation: 'Actions'
  },

  action: {
    restore: 'Restore',
    delete: 'Delete Permanently',
    preview: 'Preview'
  },

  empty: {
    title: 'Recycle Bin is Empty',
    description: 'Deleted items are kept here for 30 days'
  },

  message: {
    restoreSuccess: 'Restored successfully',
    restoreFailed: 'Restore failed',
    deleteSuccess: 'Permanently deleted',
    deleteConfirm: 'Are you sure you want to permanently delete "{name}"? This cannot be undone!',
    emptyConfirm: 'Are you sure you want to empty the recycle bin? All items will be permanently deleted!',
    emptySuccess: 'Recycle bin emptied',
    expireTip: 'This item will be auto-deleted in {time}'
  },

  tip: {
    autoDelete: 'Items in recycle bin will be auto-deleted after 30 days',
    restoreHint: 'Restored items will return to their original location'
  }
}
