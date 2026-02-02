/**
 * @description Group management - English
 */
export default {
  title: 'Group Management',

  toolbar: {
    addGroup: 'New Group',
    batchDelete: 'Batch Delete',
    sortBy: 'Sort'
  },

  sort: {
    lastUpdated: 'Last Updated',
    name: 'Name',
    sortValue: 'Sort Value'
  },

  column: {
    index: '#',
    name: 'Name',
    profileCount: 'Profiles',
    sort: 'Sort',
    remark: 'Remark',
    createTime: 'Created',
    operation: 'Actions'
  },

  form: {
    addTitle: 'New Group',
    editTitle: 'Edit Group',
    name: 'Name',
    namePlaceholder: 'Enter group name',
    icon: 'Icon',
    remark: 'Remark',
    remarkPlaceholder: 'Enter remark (optional)',
    sort: 'Sort',
    sortTip: 'Lower value = higher priority'
  },

  icon: {
    folder: 'Folder',
    shopping: 'Shopping',
    campaign: 'Marketing',
    movie: 'Media',
    payments: 'Payment',
    mail: 'Mail'
  },

  detail: {
    title: 'Group Details',
    basicInfo: 'Basic Info',
    statistics: 'Statistics',
    relatedProfiles: 'Related Profiles',
    activeSessions: 'Active Sessions',
    quickActions: 'Quick Actions',
    batchProxy: 'Batch Proxy Config',
    manageExtensions: 'Manage Extensions',
    shareToTeam: 'Share to Team'
  },

  empty: {
    title: 'No Groups',
    createButton: 'New Group'
  },

  message: {
    createSuccess: 'Group created',
    updateSuccess: 'Group updated',
    deleteSuccess: 'Group deleted',
    deleteConfirm: 'Are you sure you want to delete group "{name}"?',
    defaultGroupTip: 'Default group cannot be deleted',
    hasProfiles: 'This group has {count} profiles. Please move or delete them first.',
    batchDeleteConfirm: 'Are you sure you want to delete {count} groups? This cannot be undone!'
  },

  createTip: 'After creating a group, you can move existing profiles to it or create new ones directly in it.'
}
