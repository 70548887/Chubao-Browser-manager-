/**
 * @description 태그 관리 - 한국어
 */
export default {
  // 페이지 제목
  title: '태그 관리',

  // 도구 모음
  toolbar: {
    addTag: '태그 추가',
    batchDelete: '일괄 삭제'
  },

  // 테이블 열
  column: {
    name: '태그 이름',
    color: '색상',
    profileCount: '창 수',
    createTime: '생성 시간',
    operation: '작업'
  },

  // 추가/편집 폼
  form: {
    addTitle: '태그 추가',
    editTitle: '태그 편집',
    name: '태그 이름',
    namePlaceholder: '태그 이름을 입력하세요',
    color: '태그 색상',
    selectColor: '색상 선택'
  },

  // 색상 옵션
  color: {
    blue: '파랑',
    green: '초록',
    orange: '주황',
    red: '빨강',
    purple: '보라',
    pink: '분홍',
    cyan: '청록',
    gray: '회색'
  },

  // 빈 상태
  empty: {
    title: '태그가 없습니다',
    description: '태그를 사용하여 창을 더 효율적으로 관리하고 필터링할 수 있습니다',
    addButton: '태그 추가'
  },

  // 메시지
  message: {
    createSuccess: '태그를 만들었습니다',
    updateSuccess: '태그를 업데이트했습니다',
    deleteSuccess: '태그를 삭제했습니다',
    deleteConfirm: '태그 "{name}"을(를) 삭제하시겠습니까?',
    hasProfiles: '이 태그는 {count}개의 창에 연결되어 있습니다. 삭제 후 이 창들에서 이 태그가 제거됩니다'
  }
}
