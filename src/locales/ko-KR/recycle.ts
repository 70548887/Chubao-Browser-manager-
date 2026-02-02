/**
 * @description 휴지통 - 한국어
 */
export default {
  // 페이지 제목
  title: '휴지통',

  // 도구 모음
  toolbar: {
    restore: '복원',
    permanentDelete: '영구 삭제',
    emptyRecycle: '휴지통 비우기',
    filterByType: '유형별 필터'
  },

  // 항목 유형
  type: {
    all: '전체',
    profile: '창',
    group: '그룹',
    proxy: '프록시',
    tag: '태그',
    flow: '플로우'
  },

  // 테이블 열
  column: {
    name: '이름',
    type: '유형',
    deleteTime: '삭제 시간',
    expireTime: '만료 시간',
    deletedBy: '삭제자',
    operation: '작업'
  },

  // 작업
  action: {
    restore: '복원',
    delete: '영구 삭제',
    preview: '미리보기'
  },

  // 빈 상태
  empty: {
    title: '휴지통이 비어 있습니다',
    description: '삭제된 항목은 30일 동안 여기에 보관됩니다'
  },

  // 메시지
  message: {
    restoreSuccess: '복원되었습니다',
    restoreFailed: '복원 실패',
    deleteSuccess: '영구 삭제되었습니다',
    deleteConfirm: '"{name}"을(를) 영구 삭제하시겠습니까? 이 작업은 되돌릴 수 없습니다!',
    emptyConfirm: '휴지통을 비우시겠습니까? 모든 항목이 영구 삭제됩니다!',
    emptySuccess: '휴지통을 비웠습니다',
    expireTip: '이 항목은 {time} 후 자동 삭제됩니다'
  },

  // 힌트
  tip: {
    autoDelete: '휴지통의 항목은 삭제 후 30일이 지나면 자동으로 영구 삭제됩니다',
    restoreHint: '복원 후 항목은 원래 위치로 돌아갑니다'
  }
}
