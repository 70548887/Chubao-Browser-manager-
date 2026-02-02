/**
 * @description Tag management - English
 */
export default {
  title: 'Tag Management',

  toolbar: {
    addTag: 'Add Tag',
    batchDelete: 'Batch Delete'
  },

  column: {
    name: 'Name',
    color: 'Color',
    profileCount: 'Profiles',
    createTime: 'Created',
    operation: 'Actions'
  },

  form: {
    addTitle: 'Add Tag',
    editTitle: 'Edit Tag',
    name: 'Name',
    namePlaceholder: 'Enter tag name',
    color: 'Color',
    selectColor: 'Select Color'
  },

  color: {
    blue: 'Blue',
    green: 'Green',
    orange: 'Orange',
    red: 'Red',
    purple: 'Purple',
    pink: 'Pink',
    cyan: 'Cyan',
    gray: 'Gray'
  },

  empty: {
    title: 'No Tags',
    description: 'Tags help you organize and filter profiles',
    addButton: 'Add Tag'
  },

  message: {
    createSuccess: 'Tag created',
    updateSuccess: 'Tag updated',
    deleteSuccess: 'Tag deleted',
    deleteConfirm: 'Are you sure you want to delete tag "{name}"?',
    hasProfiles: 'This tag is used by {count} profiles. They will have this tag removed.'
  }
}
